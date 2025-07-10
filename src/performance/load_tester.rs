use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::sleep;
use serde::{Serialize, Deserialize};
use axum::http::{StatusCode, HeaderMap};
use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadTestConfig {
    pub endpoint: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub concurrent_users: usize,
    pub requests_per_user: usize,
    pub delay_between_requests: Duration,
    pub timeout: Duration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadTestResult {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub average_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub p50_response_time: Duration,
    pub p95_response_time: Duration,
    pub p99_response_time: Duration,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub status_codes: HashMap<u16, usize>,
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub struct LoadTester {
    app: axum::Router,
}

impl LoadTester {
    pub fn new(app: axum::Router) -> Self {
        Self { app }
    }
    
    pub async fn run_load_test(&self, config: LoadTestConfig) -> LoadTestResult {
        let start_time = Instant::now();
        let mut response_times = Vec::new();
        let mut status_codes = HashMap::new();
        let mut errors = Vec::new();
        let mut successful_requests = 0;
        let mut failed_requests = 0;
        
        // Create tasks for concurrent users
        let mut tasks = Vec::new();
        
        for user_id in 0..config.concurrent_users {
            let app = self.app.clone();
            let config = config.clone();
            
            let task = tokio::spawn(async move {
                let mut user_response_times = Vec::new();
                let mut user_status_codes = HashMap::new();
                let mut user_errors = Vec::new();
                let mut user_successful = 0;
                let mut user_failed = 0;
                
                for request_id in 0..config.requests_per_user {
                    let request_start = Instant::now();
                    
                    // Create request
                    let method = config.method.parse::<axum::http::Method>().unwrap_or(axum::http::Method::GET);
                    let mut request_builder = Request::builder()
                        .method(method)
                        .uri(&config.endpoint);
                    
                    // Add headers
                    for (key, value) in &config.headers {
                        request_builder = request_builder.header(key, value);
                    }
                    
                    // Add body if provided
                    let body = if let Some(body_content) = &config.body {
                        Body::from(body_content.clone())
                    } else {
                        Body::empty()
                    };
                    
                    let request = request_builder.body(body).unwrap();
                    
                    // Send request
                    let response = app.clone().oneshot(request).await;
                    
                    let response_time = request_start.elapsed();
                    user_response_times.push(response_time);
                    
                    match response {
                        Ok(response) => {
                            let status = response.status();
                            *user_status_codes.entry(status.as_u16()).or_insert(0) += 1;
                            
                            if status.is_success() {
                                user_successful += 1;
                            } else {
                                user_failed += 1;
                                user_errors.push(format!("HTTP {}: {}", status.as_u16(), status));
                            }
                        }
                        Err(e) => {
                            user_failed += 1;
                            user_errors.push(format!("Request failed: {}", e));
                        }
                    }
                    
                    // Delay between requests
                    if request_id < config.requests_per_user - 1 {
                        sleep(config.delay_between_requests).await;
                    }
                }
                
                (user_response_times, user_status_codes, user_errors, user_successful, user_failed)
            });
            
            tasks.push(task);
        }
        
        // Collect results from all tasks
        for task in tasks {
            match task.await {
                Ok((user_response_times, user_status_codes, user_errors, user_successful, user_failed)) => {
                    response_times.extend(user_response_times);
                    
                    for (status, count) in user_status_codes {
                        *status_codes.entry(status).or_insert(0) += count;
                    }
                    
                    errors.extend(user_errors);
                    successful_requests += user_successful;
                    failed_requests += user_failed;
                }
                Err(e) => {
                    errors.push(format!("Task failed: {}", e));
                    failed_requests += config.requests_per_user;
                }
            }
        }
        
        let total_requests = successful_requests + failed_requests;
        let total_time = start_time.elapsed();
        
        // Calculate statistics
        response_times.sort();
        
        let average_response_time = if !response_times.is_empty() {
            let total: Duration = response_times.iter().sum();
            total / response_times.len() as u32
        } else {
            Duration::from_millis(0)
        };
        
        let min_response_time = response_times.first().copied().unwrap_or(Duration::from_millis(0));
        let max_response_time = response_times.last().copied().unwrap_or(Duration::from_millis(0));
        
        let p50_idx = (response_times.len() as f64 * 0.5) as usize;
        let p95_idx = (response_times.len() as f64 * 0.95) as usize;
        let p99_idx = (response_times.len() as f64 * 0.99) as usize;
        
        let p50_response_time = response_times.get(p50_idx).copied().unwrap_or(Duration::from_millis(0));
        let p95_response_time = response_times.get(p95_idx).copied().unwrap_or(Duration::from_millis(0));
        let p99_response_time = response_times.get(p99_idx).copied().unwrap_or(Duration::from_millis(0));
        
        let requests_per_second = if total_time.as_secs() > 0 {
            total_requests as f64 / total_time.as_secs() as f64
        } else {
            0.0
        };
        
        let error_rate = if total_requests > 0 {
            failed_requests as f64 / total_requests as f64
        } else {
            0.0
        };
        
        LoadTestResult {
            total_requests,
            successful_requests,
            failed_requests,
            average_response_time,
            min_response_time,
            max_response_time,
            p50_response_time,
            p95_response_time,
            p99_response_time,
            requests_per_second,
            error_rate,
            status_codes,
            errors,
        }
    }
    
    pub async fn run_api_load_test(&self) -> Vec<LoadTestResult> {
        let mut results = Vec::new();
        
        // Test wallet balance endpoint
        let wallet_config = LoadTestConfig {
            endpoint: "/wallet/balance".to_string(),
            method: "GET".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Authorization".to_string(), "Bearer test_token".to_string());
                headers
            },
            body: None,
            concurrent_users: 10,
            requests_per_user: 50,
            delay_between_requests: Duration::from_millis(100),
            timeout: Duration::from_secs(30),
        };
        
        println!("Testing wallet balance endpoint...");
        let wallet_result = self.run_load_test(wallet_config).await;
        results.push(wallet_result);
        
        // Test DAG status endpoint
        let dag_config = LoadTestConfig {
            endpoint: "/dag/status".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: None,
            concurrent_users: 20,
            requests_per_user: 30,
            delay_between_requests: Duration::from_millis(50),
            timeout: Duration::from_secs(30),
        };
        
        println!("Testing DAG status endpoint...");
        let dag_result = self.run_load_test(dag_config).await;
        results.push(dag_result);
        
        // Test trading analytics endpoint
        let analytics_config = LoadTestConfig {
            endpoint: "/analytics/trading".to_string(),
            method: "GET".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Authorization".to_string(), "Bearer test_token".to_string());
                headers
            },
            body: None,
            concurrent_users: 5,
            requests_per_user: 100,
            delay_between_requests: Duration::from_millis(200),
            timeout: Duration::from_secs(30),
        };
        
        println!("Testing trading analytics endpoint...");
        let analytics_result = self.run_load_test(analytics_config).await;
        results.push(analytics_result);
        
        results
    }
}

pub fn print_load_test_report(results: &[LoadTestResult]) {
    println!("\n=== LOAD TEST REPORT ===");
    println!("Total endpoints tested: {}", results.len());
    
    for (i, result) in results.iter().enumerate() {
        println!("\n--- Endpoint {} ---", i + 1);
        println!("Total Requests: {}", result.total_requests);
        println!("Successful: {}", result.successful_requests);
        println!("Failed: {}", result.failed_requests);
        println!("Error Rate: {:.2}%", result.error_rate * 100.0);
        println!("Requests/sec: {:.2}", result.requests_per_second);
        println!("Average Response Time: {:?}", result.average_response_time);
        println!("Min Response Time: {:?}", result.min_response_time);
        println!("Max Response Time: {:?}", result.max_response_time);
        println!("P50 Response Time: {:?}", result.p50_response_time);
        println!("P95 Response Time: {:?}", result.p95_response_time);
        println!("P99 Response Time: {:?}", result.p99_response_time);
        
        if !result.status_codes.is_empty() {
            println!("Status Codes:");
            for (status, count) in &result.status_codes {
                println!("  {}: {}", status, count);
            }
        }
        
        if !result.errors.is_empty() {
            println!("Errors (first 5):");
            for error in result.errors.iter().take(5) {
                println!("  {}", error);
            }
        }
    }
    
    // Summary statistics
    let total_requests: usize = results.iter().map(|r| r.total_requests).sum();
    let total_successful: usize = results.iter().map(|r| r.successful_requests).sum();
    let total_failed: usize = results.iter().map(|r| r.failed_requests).sum();
    let avg_rps: f64 = results.iter().map(|r| r.requests_per_second).sum::<f64>() / results.len() as f64;
    
    println!("\n=== SUMMARY ===");
    println!("Total Requests: {}", total_requests);
    println!("Total Successful: {}", total_successful);
    println!("Total Failed: {}", total_failed);
    println!("Overall Error Rate: {:.2}%", (total_failed as f64 / total_requests as f64) * 100.0);
    println!("Average Requests/sec: {:.2}", avg_rps);
} 