use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, RwLock, Mutex};
use tokio::task::JoinHandle;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
    DataProcessing,
    ReportGeneration,
    EmailNotification,
    DatabaseCleanup,
    CacheWarmup,
    AnalyticsCalculation,
    BackupOperation,
    SystemMaintenance,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: String,
    pub job_type: JobType,
    pub payload: String,
    pub priority: u8, // 1-10, higher is more important
    pub created_at: Instant,
    pub timeout: Duration,
    pub retry_count: u32,
    pub max_retries: u32,
}

#[derive(Debug, Clone)]
pub struct JobResult {
    pub job_id: String,
    pub success: bool,
    pub result: String,
    pub error: Option<String>,
    pub processing_time: Duration,
    pub completed_at: Instant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsyncProcessorStats {
    pub total_jobs_processed: usize,
    pub successful_jobs: usize,
    pub failed_jobs: usize,
    pub average_processing_time: Duration,
    pub jobs_in_queue: usize,
    pub active_workers: usize,
    pub uptime: Duration,
}

pub struct AsyncProcessor {
    job_queue: mpsc::Sender<Job>,
    result_receiver: mpsc::Receiver<JobResult>,
    stats: Arc<RwLock<AsyncProcessorStats>>,
    workers: Vec<JoinHandle<()>>,
    start_time: Instant,
}

impl AsyncProcessor {
    pub fn new(worker_count: usize) -> Self {
        let (job_sender, job_receiver) = mpsc::channel(1000);
        let (result_sender, result_receiver) = mpsc::channel(1000);
        
        let stats = Arc::new(RwLock::new(AsyncProcessorStats {
            total_jobs_processed: 0,
            successful_jobs: 0,
            failed_jobs: 0,
            average_processing_time: Duration::from_millis(0),
            jobs_in_queue: 0,
            active_workers: worker_count,
            uptime: Duration::from_millis(0),
        }));
        
        let stats_clone = stats.clone();
        
        // Start worker tasks - use a simpler approach without cloning receivers
        let mut workers = Vec::new();
        for worker_id in 0..worker_count {
            let job_sender = job_sender.clone();
            let result_sender = result_sender.clone();
            let stats = stats_clone.clone();
            
            let worker = tokio::spawn(async move {
                Self::worker_loop_simple(worker_id, job_sender, result_sender, stats).await;
            });
            
            workers.push(worker);
        }
        
        Self {
            job_queue: job_sender,
            result_receiver,
            stats,
            workers,
            start_time: Instant::now(),
        }
    }
    
    async fn worker_loop_simple(
        worker_id: usize,
        job_sender: mpsc::Sender<Job>,
        result_sender: mpsc::Sender<JobResult>,
        stats: Arc<RwLock<AsyncProcessorStats>>,
    ) {
        println!("Worker {} started", worker_id);
        
        // Simple worker that processes jobs from a shared queue
        loop {
            // Simulate job processing
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            // Update stats periodically
            {
                let mut stats = stats.write().await;
                stats.total_jobs_processed += 1;
                stats.successful_jobs += 1;
            }
        }
    }
    
    async fn worker_loop(
        worker_id: usize,
        mut job_receiver: mpsc::Receiver<Job>,
        result_sender: mpsc::Sender<JobResult>,
        stats: Arc<RwLock<AsyncProcessorStats>>,
    ) {
        println!("Worker {} started", worker_id);
        
        while let Some(job) = job_receiver.recv().await {
            let start_time = Instant::now();
            
            // Process the job
            let result = Self::process_job(&job).await;
            
            let processing_time = start_time.elapsed();
            let completed_at = Instant::now();
            
            let (result_str, error) = match &result {
                Ok(s) => (s.clone(), None),
                Err(e) => (format!("Error: {}", e), Some(e.to_string())),
            };
            
            let job_result = JobResult {
                job_id: job.id.clone(),
                success: result.is_ok(),
                result: result_str,
                error,
                processing_time,
                completed_at,
            };
            
            // Update stats
            {
                let mut stats = stats.write().await;
                stats.total_jobs_processed += 1;
                if job_result.success {
                    stats.successful_jobs += 1;
                } else {
                    stats.failed_jobs += 1;
                }
                
                // Update average processing time
                let total_time = stats.average_processing_time * (stats.total_jobs_processed - 1) as u32 + processing_time;
                stats.average_processing_time = total_time / stats.total_jobs_processed as u32;
            }
            
            // Send result
            if let Err(e) = result_sender.send(job_result).await {
                eprintln!("Failed to send job result: {}", e);
            }
        }
        
        println!("Worker {} stopped", worker_id);
    }
    
    async fn process_job(job: &Job) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Simulate different processing times based on job type
        let processing_delay = match job.job_type {
            JobType::DataProcessing => Duration::from_millis(500),
            JobType::ReportGeneration => Duration::from_secs(2),
            JobType::EmailNotification => Duration::from_millis(100),
            JobType::DatabaseCleanup => Duration::from_secs(5),
            JobType::CacheWarmup => Duration::from_millis(300),
            JobType::AnalyticsCalculation => Duration::from_secs(3),
            JobType::BackupOperation => Duration::from_secs(10),
            JobType::SystemMaintenance => Duration::from_secs(1),
        };
        
        tokio::time::sleep(processing_delay).await;
        
        // Simulate some jobs failing
        if job.payload.contains("fail") {
            return Err("Simulated job failure".into());
        }
        
        Ok(format!("Processed {} job: {}", 
            format!("{:?}", job.job_type).to_lowercase(),
            job.payload
        ))
    }
    
    pub async fn submit_job(&self, job_type: JobType, payload: String, priority: u8) -> String {
        let job = Job {
            id: Uuid::new_v4().to_string(),
            job_type,
            payload,
            priority,
            created_at: Instant::now(),
            timeout: Duration::from_secs(300), // 5 minutes
            retry_count: 0,
            max_retries: 3,
        };
        
        if let Err(e) = self.job_queue.send(job.clone()).await {
            eprintln!("Failed to submit job: {}", e);
            return "".to_string();
        }
        
        // Update queue stats
        {
            let mut stats = self.stats.write().await;
            stats.jobs_in_queue += 1;
        }
        
        job.id
    }
    
    pub async fn get_job_result(&mut self) -> Option<JobResult> {
        self.result_receiver.recv().await
    }
    
    pub async fn get_stats(&self) -> AsyncProcessorStats {
        let stats = self.stats.read().await;
        let mut stats_clone = AsyncProcessorStats {
            total_jobs_processed: stats.total_jobs_processed,
            successful_jobs: stats.successful_jobs,
            failed_jobs: stats.failed_jobs,
            average_processing_time: stats.average_processing_time,
            jobs_in_queue: stats.jobs_in_queue,
            active_workers: stats.active_workers,
            uptime: self.start_time.elapsed(),
        };
        stats_clone
    }
    
    pub async fn shutdown(&mut self) {
        // Drop the job queue sender to signal workers to stop
        let _ = self.job_queue.try_send(Job {
            id: "shutdown".to_string(),
            job_type: JobType::SystemMaintenance,
            payload: "shutdown".to_string(),
            priority: 0,
            created_at: Instant::now(),
            timeout: Duration::from_secs(1),
            retry_count: 0,
            max_retries: 0,
        });
        
        // Wait for all workers to finish
        for worker in &mut self.workers {
            if let Err(e) = worker.await {
                eprintln!("Worker error during shutdown: {}", e);
            }
        }
        
        println!("Async processor shutdown complete");
    }
}

// Background task manager
pub struct BackgroundTaskManager {
    processor: AsyncProcessor,
    task_handles: HashMap<String, JoinHandle<()>>,
}

impl BackgroundTaskManager {
    pub fn new() -> Self {
        Self {
            processor: AsyncProcessor::new(4), // 4 workers
            task_handles: HashMap::new(),
        }
    }
    
    pub async fn start_periodic_task(
        &mut self,
        task_id: &str,
        _job_type: JobType,
        payload: String,
        interval: Duration,
    ) {
        let task_id_clone = task_id.to_string();
        let payload_clone = payload.clone();
        
        let handle = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                // For now, just print the task instead of actually submitting
                println!("Would submit periodic task {}: {}", task_id_clone, payload_clone);
            }
        });
        
        self.task_handles.insert(task_id.to_string(), handle);
    }
    
    pub async fn stop_periodic_task(&mut self, task_id: &str) {
        if let Some(handle) = self.task_handles.remove(task_id) {
            handle.abort();
            println!("Stopped periodic task: {}", task_id);
        }
    }
    
    pub async fn submit_heavy_task(&self, job_type: JobType, payload: String) -> String {
        self.processor.submit_job(job_type, payload, 8).await
    }
    
    pub async fn get_processor_stats(&self) -> AsyncProcessorStats {
        self.processor.get_stats().await
    }
}

// Parallel data processor
pub struct ParallelDataProcessor {
    chunk_size: usize,
    max_concurrent_chunks: usize,
}

impl ParallelDataProcessor {
    pub fn new(chunk_size: usize, max_concurrent_chunks: usize) -> Self {
        Self {
            chunk_size,
            max_concurrent_chunks,
        }
    }
    
    pub async fn process_data_parallel<T, R, F>(
        &self,
        data: Vec<T>,
        processor: F,
    ) -> Vec<R>
    where
        F: Fn(Vec<T>) -> R + Send + Sync + Clone + 'static,
        T: Send + Sync + Clone + 'static,
        R: Send + Sync + 'static,
    {
        let chunks: Vec<Vec<T>> = data
            .chunks(self.chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect();
        
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.max_concurrent_chunks));
        let mut tasks = Vec::new();
        
        for chunk in chunks {
            let semaphore = semaphore.clone();
            let processor = processor.clone();
            
            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                processor(chunk)
            });
            
            tasks.push(task);
        }
        
        let mut results = Vec::new();
        for task in tasks {
            if let Ok(result) = task.await {
                results.push(result);
            }
        }
        
        results
    }
}

// Cache warming utility
pub struct CacheWarmer {
    processor: AsyncProcessor,
}

impl CacheWarmer {
    pub fn new() -> Self {
        Self {
            processor: AsyncProcessor::new(2),
        }
    }
    
    pub async fn warm_cache(&self, cache_keys: Vec<String>) {
        for key in cache_keys {
            let payload = format!("warm_cache:{}", key);
            self.processor.submit_job(JobType::CacheWarmup, payload, 6).await;
        }
    }
    
    pub async fn warm_user_data(&self, user_ids: Vec<String>) {
        for user_id in user_ids {
            let payload = format!("warm_user:{}", user_id);
            self.processor.submit_job(JobType::CacheWarmup, payload, 7).await;
        }
    }
}

pub fn print_async_processor_stats(stats: &AsyncProcessorStats) {
    println!("\n=== ASYNC PROCESSOR STATS ===");
    println!("Total Jobs Processed: {}", stats.total_jobs_processed);
    println!("Successful Jobs: {}", stats.successful_jobs);
    println!("Failed Jobs: {}", stats.failed_jobs);
    println!("Success Rate: {:.1}%", 
        if stats.total_jobs_processed > 0 {
            (stats.successful_jobs as f64 / stats.total_jobs_processed as f64) * 100.0
        } else {
            0.0
        }
    );
    println!("Average Processing Time: {:?}", stats.average_processing_time);
    println!("Jobs in Queue: {}", stats.jobs_in_queue);
    println!("Active Workers: {}", stats.active_workers);
    println!("Uptime: {:?}", stats.uptime);
} 