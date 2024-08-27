// pub  mod remote;
pub mod immutable_agent;
pub mod llama_structs;
pub mod llm_utils;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LlmConfig {
    pub model: &'static str,
    pub base_url: &'static str,
    pub context_size: usize,
    pub api_key_str: &'static str,
}

pub const GROQ_LOCAL_CONFIG: LlmConfig = LlmConfig {
    model: "llama3",
    context_size: 8192,
    base_url: "http://127.0.0.1:8080/v1/chat/completions",
    api_key_str: "None",
};

pub const TOGETHER_CONFIG: LlmConfig = LlmConfig {
    model: "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo",
    context_size: 8192,
    base_url: "https://api.together.xyz/v1/chat/completions",
    api_key_str: "TOGETHER_API_KEY",
};

pub const CODELLAMA_CONFIG: LlmConfig = LlmConfig {
    model: "codellama/CodeLlama-34b-Instruct-hf",
    context_size: 8192,
    base_url: "https://api.together.xyz/v1/chat/completions",
    api_key_str: "TOGETHER_API_KEY",
};

pub const QWEN_CONFIG: LlmConfig = LlmConfig {
    model: "Qwen/Qwen2-72B-Instruct",
    context_size: 32000,
    base_url: "https://api.deepinfra.com/v1/openai/chat/completions",
    api_key_str: "DEEPINFRA_API_KEY",
};

pub const DEEPSEEK_CONFIG: LlmConfig = LlmConfig {
    model: "deepseek-coder",
    context_size: 16000,
    base_url: "https://api.deepseek.com/chat/completions",
    api_key_str: "SEEK_API_KEY",
};
