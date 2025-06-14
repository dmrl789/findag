use serde::{Serialize, Deserialize};
use crate::types::finality::Justification;
use crate::ai::{generate_response, AIRequest};
use crate::types::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockAnalysis {
    pub relevance_score: f32,
    pub category: String,
    pub key_topics: Vec<String>,
    pub summary: String,
    pub suggested_improvements: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub hash_timer: u64,
    pub content: Vec<Transaction>,
    pub justification: Option<Justification>,
    pub ai_analysis: Option<BlockAnalysis>,
}

impl Block {
    pub fn new(content: Vec<Transaction>) -> Self {
        Self {
            hash_timer: crate::utils::time::get_findag_time_micro(),
            content,
            justification: None,
            ai_analysis: None,
        }
    }

    pub async fn analyze(&mut self) -> Result<(), String> {
        let prompt = format!(
            "Analyze the following block content and provide a detailed analysis:\n\
             Content: {}\n\
             \n\
             Please provide:\n\
             1. A relevance score (0-1)\n\
             2. A category\n\
             3. Key topics\n\
             4. A brief summary\n\
             5. Suggested improvements\n\
             Format the response as JSON with these fields: relevance_score, category, key_topics, summary, suggested_improvements",
            self.content
        );

        let request = AIRequest {
            data: "placeholder_data".to_string(),
            prompt,
            model: Some("gpt-3.5-turbo".to_string()),
            temperature: Some(0.3),
            max_tokens: Some(500),
        };

        match generate_response(request).await {
            Ok(response) if response.error.is_none() => {
                match serde_json::from_str::<BlockAnalysis>(&response.text) {
                    Ok(analysis) => {
                        self.ai_analysis = Some(analysis);
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to parse AI response: {}", e)),
                }
            }
            Ok(response) => Err(response.error.unwrap_or_else(|| "Unknown error".to_string())),
            Err(e) => Err(format!("AI analysis failed: {}", e)),
        }
    }

    pub fn get_relevance_score(&self) -> Option<f32> {
        self.ai_analysis.as_ref().map(|analysis| analysis.relevance_score)
    }

    pub fn get_category(&self) -> Option<&str> {
        self.ai_analysis.as_ref().map(|analysis| analysis.category.as_str())
    }

    pub fn get_summary(&self) -> Option<&str> {
        self.ai_analysis.as_ref().map(|analysis| analysis.summary.as_str())
    }

    pub fn get_suggested_improvements(&self) -> Option<&Vec<String>> {
        self.ai_analysis.as_ref().map(|analysis| &analysis.suggested_improvements)
    }
}
