import { OpenAI } from 'openai';
import { config } from '../config';

// Initialize OpenAI client
const openai = new OpenAI({
  apiKey: config.openaiApiKey,
});

// Types
export interface AIResponse {
  text: string;
  error?: string;
}

export interface AIRequest {
  prompt: string;
  model?: string;
  temperature?: number;
  maxTokens?: number;
}

// Default configuration
const DEFAULT_MODEL = 'gpt-3.5-turbo';
const DEFAULT_TEMPERATURE = 0.7;
const DEFAULT_MAX_TOKENS = 1000;

/**
 * Generate a response from the AI model
 */
export async function generateResponse(request: AIRequest): Promise<AIResponse> {
  try {
    const response = await openai.chat.completions.create({
      model: request.model || DEFAULT_MODEL,
      messages: [{ role: 'user', content: request.prompt }],
      temperature: request.temperature || DEFAULT_TEMPERATURE,
      max_tokens: request.maxTokens || DEFAULT_MAX_TOKENS,
    });

    return {
      text: response.choices[0]?.message?.content || '',
    };
  } catch (error) {
    console.error('Error generating AI response:', error);
    return {
      text: '',
      error: error instanceof Error ? error.message : 'Unknown error occurred',
    };
  }
}

/**
 * Validate if the AI service is properly configured
 */
export function isAIConfigured(): boolean {
  return !!config.openaiApiKey;
} 