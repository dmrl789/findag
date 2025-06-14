use findag_core::blockchain::{Block, Blockchain};
use findag_core::storage::Storage;

#[tokio::test]
async fn test_block_analysis() {
    // Skip test if API key is not configured
    if !findag_core::ai::is_ai_configured() {
        println!("Skipping blockchain AI test - OPENAI_API_KEY not configured");
        return;
    }

    let content = "This is a test block about artificial intelligence and blockchain technology. \
                  It discusses the integration of AI with distributed systems and smart contracts.";
    
    let mut block = Block::new(content.to_string());
    
    // Test block analysis
    let analysis_result = block.analyze().await;
    assert!(analysis_result.is_ok(), "Block analysis failed: {:?}", analysis_result.err());
    
    let analysis = block.ai_analysis.unwrap();
    assert!(analysis.relevance_score >= 0.0 && analysis.relevance_score <= 1.0);
    assert!(!analysis.category.is_empty());
    assert!(!analysis.key_topics.is_empty());
    assert!(!analysis.summary.is_empty());
    
    // Test getter methods
    assert!(block.get_relevance_score().is_some());
    assert!(block.get_category().is_some());
    assert!(block.get_summary().is_some());
    assert!(block.get_suggested_improvements().is_some());
}

#[tokio::test]
async fn test_blockchain_ai_features() {
    // Skip test if API key is not configured
    if !findag_core::ai::is_ai_configured() {
        println!("Skipping blockchain AI features test - OPENAI_API_KEY not configured");
        return;
    }

    let storage = Storage::new_in_memory();
    let blockchain = Blockchain::new(storage);

    // Test content analysis
    let content = "A detailed analysis of blockchain consensus mechanisms and their security implications";
    let analysis = blockchain.analyze_block_content(content).await;
    assert!(analysis.is_ok(), "Content analysis failed: {:?}", analysis.err());
    
    let analysis = analysis.unwrap();
    assert!(analysis.relevance_score > 0.5);
    assert!(analysis.category.to_lowercase().contains("blockchain"));
    assert!(!analysis.key_topics.is_empty());
    
    // Test block addition with analysis
    let block = Block::new(content.to_string());
    let add_result = blockchain.add_block(block).await;
    assert!(add_result.is_ok(), "Block addition failed: {:?}", add_result.err());
} 