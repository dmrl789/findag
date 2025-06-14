use findag_core::ai::{generate_response, AIRequest};

#[tokio::test]
async fn test_ai_generation() {
    // Skip test if API key is not configured
    if !findag_core::ai::is_ai_configured() {
        println!("Skipping AI test - OPENAI_API_KEY not configured");
        return;
    }

    let request = AIRequest {
        prompt: "What is the capital of France?".to_string(),
        model: None,
        temperature: None,
        max_tokens: None,
    };

    let response = generate_response(request).await;
    assert!(response.error.is_none(), "Error: {:?}", response.error);
    assert!(!response.text.is_empty(), "Response text is empty");
    println!("AI Response: {}", response.text);
} 