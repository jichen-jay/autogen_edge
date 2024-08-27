use autogen_edge::llama_structs::*;
use autogen_edge::llm_utils::*;
use autogen_edge::{GROQ_LOCAL_CONFIG, TOGETHER_CONFIG};
// use async_openai::{
//     types::{
//         ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
//         ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
//     },
//     Client,
// };

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();

    let tools = r#"You are a function calling AI model. You are provided with function signatures within <tools></tools> XML tags. You may call one or more functions to assist with the user query. Don't make assumptions about what values to plug into functions. For each function call return a json object with function name and arguments within <tool_call></tool_call> XML tags as follows:
<tool_call>
{"name": <function-name>,"arguments": <args-dict>}
</tool_call>

Here are the available tools:
<tools> {
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
} </tools>"#;

    let tool = serde_json::json!(
        r#"        {
        "type": "function",
        "function": {
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
    }
"#
    );

    let prompt = r#"{
    "model": "llama3",
    "messages": [
        {
            "role": "user",
            "content": "what's the weather in San Francisco in Celsius"
        }
    ],
    "temperature": 0.8,
    "top_p": 1.0,
    "n": 3,
    "stream": false,
    "max_tokens": 500,
    "presence_penalty": 0.5,
    "frequency_penalty": 0.5,
    "response_format": {
        "type": "text"
    },
    "tools": [
        {
            "type": "function",
            "function": {
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
        }
    ],
    "tool_choice": {
        "type": "function",
        "function": {
            "name": "get_current_weather"
        }
    }
}"#;

    let response = chat_inner_tool_call(&GROQ_LOCAL_CONFIG, &prompt, 500)
        .await
        .expect("bad response");

    println!("\nResponse:\n");
    println!(" {:?}", response.content);
    // let prompt = create_system_prompt_tool(tools)?;
    //
    // println!("prompt: {:?}", prompt.clone());

    Ok(())
}

async fn save() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();

    let max_token = 500u16;
    let output: LlamaResponseMessage = chat_inner_async_wrapper(
        &TOGETHER_CONFIG,
        "you're AI assistant",
        "tell me a joke",
        max_token,
    )
    .await
    .expect("Failed to generate reply");

    match &output.content {
        Content::Text(_out) => {
            println!("sub_tasks: {:?}", _out,);
        }
        _ => unreachable!(),
    }

    //
    // println!("PUT: {}", body);

    Ok(())
}

// #[test]
// fn test_serialize_chat_completion_object_message() {
//     let tool = ToolCall {
//         id: "call_abc123".to_string(),
//         ty: "function".to_string(),
//         function: Function {
//             name: "get_current_weather".to_string(),
//             arguments: "{\"location\": \"Boston, MA\"}".to_string(),
//         },
//     };
//     let message = ChatCompletionObjectMessage {
//         content: None,
//         tool_calls: vec![tool],
//         role: ChatCompletionRole::Assistant,
//         function_call: None,
//     };
//     let json = serde_json::to_string(&message).unwrap();
//     assert_eq!(
//         json,
//         r#"{"content":null,"tool_calls":[{"id":"call_abc123","type":"function","function":{"name":"get_current_weather","arguments":"{\"location\": \"Boston, MA\"}"}}],"role":"assistant"}"#
//     );
// }
//
// #[test]
// fn test_deserialize_chat_completion_object_message() {
//     {
//         let json = r#"{"content":null,"tool_calls":[{"id":"call_abc123","type":"function","function":{"name":"get_current_weather","arguments":"{\"location\": \"Boston, MA\"}"}}],"role":"assistant"}"#;
//         let message: ChatCompletionObjectMessage = serde_json::from_str(json).unwrap();
//         assert_eq!(message.content, None);
//         assert_eq!(message.tool_calls.len(), 1);
//         assert_eq!(message.role, ChatCompletionRole::Assistant);
//     }
//
//     {
//         let json = r#"{"content":null,"role":"assistant"}"#;
//         let message: ChatCompletionObjectMessage = serde_json::from_str(json).unwrap();
//         assert_eq!(message.content, None);
//         assert!(message.tool_calls.is_empty());
//         assert_eq!(message.role, ChatCompletionRole::Assistant);
//     }
// }
