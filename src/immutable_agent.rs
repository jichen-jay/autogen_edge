use crate::llama_structs::*;
use crate::llm_utils::*;
// use crate::webscraper_hook::{get_webpage_text, search_with_bing};
use crate::{
    TOGETHER_CONFIG,
};
use async_openai::types::Role;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub  const GROQ_TOOLCALL: &'static str = r#"
{
    "name": "get_current_weather",
    "description": "Get the current weather in a given location",
    "parameters": {
        "properties": {
            "location": {
                "description": "The city and state, e.g. San Francisco, CA",
                "type": "string"
            },
            "unit": {
                "enum": [
                    "celsius",
                    "fahrenheit"
                ],
                "type": "string"
            }
        },
        "required": [
            "location"
        ],
        "type": "object"
    }
}
"#;

pub  const NEXT_STEP_BY_TOOLCALL: &'static str = r#"
<|im_start|>system You are a function-calling AI model. You are provided with function signatures within <tools></tools> XML tags. You will call one function and ONLY ONE to assist with the user query. Do not make assumptions about what values to plug into functions.

<tools>
1. **use_intrinsic_knowledge**: 
Description: Solves tasks using capabilities and knowledge obtained at training time.
Special Note: You can handle many fuzzy tasks this way because you have great writing skills, you may provide a common sense solution despite you might not know the exact details. 
Example Call:
<tool_call>
{"arguments": {"task": "tell a joke"}, 
"name": "use_intrinsic_knowledge"}
</tool_call>

2. **search_with_bing**: 
Description: Conducts an internet search using Bing search engine and returns relevant results based on the query provided by the user.
Special Note 1: This function helps narrow down potential sources of information before extracting specific content.
Special Note 2: Using this as an initial step can make subsequent tasks more targeted by providing exact links that can then be scraped using get_webpage_text.
Example Call:
<tool_call>
{"arguments": {"query": "latest AI research trends"}, 
"name": "search_with_bing"}
</tool_call>

3. **code_with_python**: 
Description: Generates clean, executable Python code for various tasks based on user input.
Special Note: When task requires precise mathematical operations; processing, analyzing and creating complex data types, where AI models can not efficiently represent and manipulate in natural language terms, this is the way out.
Example Call:
<tool_call>
{"arguments": {"key_points": "Create a Python script that reads a CSV file and plots a graph"}, 
"name": "code_with_python"}
</tool_call>
</tools>

Function Definitions

use_intrinsic_knowledge
Description: Solves tasks using built-in capabilities obtained at training time.
Parameters: "task" The task you receive (type:string)
Required Parameters: ["task"]

search_with_bing
Description: Conducts an internet search using Bing search engine and returns relevant results based on the query provided by the user.
Parameters: "query" The search query to be executed on Bing (type:string)
Required Parameters: ["query"]

code_with_python
Description: Generates clean executable Python code for various tasks based on key points describing what needs to be solved with code.
Parameters: "key_points" Key points describing what kind of problem needs to be solved with Python code (type:string)
Required Parameters: ["key_points"]

Remember that you are a dispatcher; you DO NOT work on tasks yourself, especially when you see specific coding suggestions, don't write any code, just dispatch.

For each function call, return a JSON object with function name and arguments within <tool_call></tool_call> XML tags as follows:

<tool_call>  
{"arguments": <args-dict>,   
"name": "<function_name>"}
</tool_call>
"#;

const ITERATE_NEXT_STEP: &'static str = r#"
<|im_start|>system You are a task solving expert. You follow steps to solve complex problems. For much of the time, you're working iteratively on the sub_tasks, you are given the result from a previous step, you execute on the instruction you receive for your current step.
"#;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub content: Content,
    pub name: Option<String>,
    pub role: Role,
}

impl Default for Message {
    fn default() -> Self {
        Message {
            content: Content::Text("placeholder".to_string()),
            name: None,
            role: Role::User,
        }
    }
}

impl Message {
    pub fn new(content: Content, name: Option<String>, role: Role) -> Self {
        Message {
            content,
            name,
            role, // Set default role to Assistant if None is provided
        }
    }
}

pub struct ImmutableAgent {
    pub name: String,
    pub system_prompt: String,
    pub llm_config: Option<Value>,
    pub tools_map_meta: String,
    pub description: String,
}

impl ImmutableAgent {
    pub fn simple(name: &str, system_prompt: &str) -> Self {
        ImmutableAgent {
            name: name.to_string(),
            system_prompt: system_prompt.to_string(),
            llm_config: None,
            tools_map_meta: String::from(""),
            description: String::from(""),
        }
    }

    pub fn new(
        name: &str,
        system_prompt: &str,
        llm_config: Option<Value>,
        tools_map_meta: &str,
        description: &str,
    ) -> Self {
        ImmutableAgent {
            name: name.to_string(),
            system_prompt: system_prompt.to_string(),
            llm_config,
            tools_map_meta: tools_map_meta.to_string(),
            description: description.to_string(),
        }
    }

    pub async fn next_step_by_toolcall(
        &self,
        carry_over: Option<String>,
        input: &str,
    ) -> anyhow::Result<String> {
        let max_token = 1000u16;
        let output: LlamaResponseMessage =
            chat_inner_async_wrapper(&TOGETHER_CONFIG, NEXT_STEP_BY_TOOLCALL, input, max_token)
                .await
                .expect("Failed to generate reply");
        match &output.content {
            Content::Text(unexpected_result) => {
                return Ok(format!(
                    "attempt to run tool_call failed, returning text result: {} ",
                    unexpected_result
                ));
            }
            Content::ToolCall(call) => {
                let args = call.clone().arguments.unwrap_or_default();
                let res = match call.name.as_str() {
                    "use_intrinsic_knowledge" => match args.get("task") {
                        Some(t) => {
                            println!("entered intrinsic arm: {:?}", t.clone());
                        }
                        None => (),
                    },
                    _ => {
                        panic!();
                    }
                };
                Ok(String::new())
            }
        }
    }



}

