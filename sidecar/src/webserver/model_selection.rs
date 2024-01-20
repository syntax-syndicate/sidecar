//! Contains the types for model selection which we want to use

use llm_client::{
    clients::types::LLMType,
    provider::{LLMProvider, LLMProviderAPIKeys},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LLMClientConfig {
    pub slow_model: LLMType,
    pub fast_model: LLMType,
    pub models: HashMap<LLMType, Model>,
    pub providers: Vec<LLMProviderAPIKeys>,
}

impl LLMClientConfig {
    pub fn provider_for_slow_model(&self) -> Option<&LLMProviderAPIKeys> {
        // we first need to get the model configuration for the slow model
        // which will give us the model and the context around it
        let model = self.models.get(&self.slow_model);
        if let None = model {
            return None;
        }
        let model = model.expect("is_none above to hold");
        let provider = &model.provider;
        // get the related provider if its present
        self.providers.iter().find(|p| p.key(provider).is_some())
    }

    pub fn provider_for_fast_model(&self) -> Option<&LLMProviderAPIKeys> {
        // we first need to get the model configuration for the slow model
        // which will give us the model and the context around it
        let model = self.models.get(&self.fast_model);
        if let None = model {
            return None;
        }
        let model = model.expect("is_none above to hold");
        let provider = &model.provider;
        // get the related provider if its present
        self.providers.iter().find(|p| p.key(provider).is_some())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Model {
    pub context_length: u32,
    pub temperature: f32,
    pub provider: LLMProvider,
}

#[cfg(test)]
mod tests {
    use super::LLMClientConfig;

    #[test]
    fn test_json_should_convert_properly() {
        let data = r#"
        {
			"slow_model": "GPT3_5_16k",
			"fast_model": "GPT3_5_16k",
			"models": {
				"GPT3_5_16k": {
					"context_length": 16000,
					"temperature": 0.2,
					"provider": "Azure"
				}
			},
			"providers": [
				{
					"OpenAIAzureConfig": {
						"deployment_id": "gpt35-turbo-access",
						"api_base": "https://codestory-gpt4.openai.azure.com",
						"api_key": "89ca8a49a33344c9b794b3dabcbbc5d0"
					}
				}
			]
		}
        "#;
        assert!(serde_json::from_str::<LLMClientConfig>(data).is_ok());
    }
}
