#[cfg(test)]
mod tests {
    use langhub::{LLMClient, LLMConfig, types::ModelProvider};

    use super::*;

    #[tokio::test]
    async fn test_openai_client_creation() {
        let config = LLMConfig::new().openai("test-api-key".to_string());
        let client = LLMClient::new_with_config(ModelProvider::OpenAI, &config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_deepseek_client_creation() {
        let config = LLMConfig::new().deepseek("test-api-key".to_string());
        let client = LLMClient::new_with_config(ModelProvider::DeepSeek, &config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_anthropic_client_creation() {
        let config = LLMConfig::new().anthropic("test-api-key".to_string());
        let client = LLMClient::new_with_config(ModelProvider::Anthropic, &config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_google_client_creation() {
        let config = LLMConfig::new().google("test-api-key".to_string());
        let client = LLMClient::new_with_config(ModelProvider::Google, &config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_missing_api_key_error() {
        let config = LLMConfig::new();
        let client = LLMClient::new_with_config(ModelProvider::OpenAI, &config);
        assert!(client.is_err());
    }
}
