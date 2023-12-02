// This is a simple rust program that calls the open ai chat gpt api
use reqwest;
use reqwest_eventsource::EventSource;
use crate::cli;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OpenAiMessageObject{
    pub role: String,
    pub content: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OpenAiChatCompletionResponseUsage{
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OpenAiChatCompletionResponseChoice {
    pub index: i64,
    pub message: OpenAiMessageObject,
    pub finish_reason: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct OpenAiChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<OpenAiChatCompletionResponseChoice>,
    pub usage: OpenAiChatCompletionResponseUsage,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct OpenAiChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OpenAiMessageObject>,
    pub response_format: ResponseFormat,
    pub stream: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ResponseFormat{
    pub r#type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CommandSuggestionResponse{
    pub message: String,
    pub command_suggestion: String,
}

pub async fn get_response_object(prompt: String, openai_api_key: String, options: cli::CommandLineOptions) -> Result<CommandSuggestionResponse, serde_json::Error> {
    let api_response = match get_openai_response_stream(prompt, openai_api_key, options).await {
        Ok(res) => {res},
        Err(e) => {
            println!("Error: {}", e);
            "{}".to_string()
        },
    };

    let command_suggestion_response = match serde_json::from_str::<OpenAiChatCompletionResponse>(&api_response) {
        Ok(js) => {
            match serde_json::from_str::<CommandSuggestionResponse>(js.choices[0].message.content.as_str()){
                Ok(js) => {Ok(js)},
                Err(e) => {
                    println!("Error when parsing command suggestion response: {}", e);
                    Err(e)
                },
            }
        },
        Err(e) => {
            println!("Error when getting response from api. Make sure you've added your secret key to your environment ('OPENAI_API_KEY=sk_....'): {}", e);
            Err(e)
        },
    };

    command_suggestion_response

    
}

pub async fn get_openai_response(prompt: String, openai_api_key: String, options: cli::CommandLineOptions) -> Result<String, reqwest::Error>{
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));
    headers.append("Authorization", reqwest::header::HeaderValue::from_str(&format!("Bearer {}", openai_api_key)).unwrap());

    let body = OpenAiChatCompletionRequest {
        model: "gpt-4-1106-preview".to_string(),
        messages: vec![
            OpenAiMessageObject {
                role: "system".to_string(),
                content: "You are the personification of a user's command line interface. You are helpful, friendly, and extremely knowledgable. You exist in the context of an Arch Linux terminal emulator. If the user is asking how to do something in the terminal, you will respond with a linux command line command to help them complete their task. Always return responses with a JSON object, with the fields 'message' and 'command_suggestion'. With your text response being the message, and the command_suggestion being the command string you are suggesting they run. If there is no command to suggest, return 'command_suggestion' as 'NULL'.".to_string(),
            },
            OpenAiMessageObject {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        // Response format is a json object with the field 'type' and the value 'json_object'
        response_format: ResponseFormat{
            r#type: "json_object".to_string(),
        },
        stream: false
    };

    let res = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    // Print the response
    if options.debug {
        println!("API Response: {:?}", res);
    }

    let response_text = res.text().await?;

    Ok(response_text)
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OpenAiResponseByteStream{
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<OpenAiResponseByteStreamChoice>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OpenAiResponseByteStreamChoice{
    pub index: u32,
    pub delta: OpenAiResponseByteStreamChoiceDelta,
    pub finish_reason: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OpenAiResponseByteStreamChoiceDelta{
    pub content: Option<String>,
}
use futures_util::StreamExt;
use reqwest_eventsource::Event;
use std::io::{stdout, Write};

use crossterm::{
    cursor,
    execute,
    terminal::{Clear, ClearType},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    event,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct StreamCommandSuggestionResponse{
    pub message: String,
    pub command_suggestion: String,
}

pub async fn get_stream_response_object(prompt: String, openai_api_key: String, options: cli::CommandLineOptions) -> Result<CommandSuggestionResponse, serde_json::Error> {
    let api_response = match get_openai_response_stream(prompt, openai_api_key, options).await {
        Ok(res) => {res},
        Err(e) => {
            println!("Error: {}", e);
            "{}".to_string()
        },
    };

    let command_suggestion_response = match serde_json::from_str::<OpenAiChatCompletionResponse>(&api_response) {
        Ok(js) => {
            match serde_json::from_str::<StreamCommandSuggestionResponse>(format!{
                "{}
                ", 
            }){
                Ok(js) => {Ok(js)},
                Err(e) => {
                    println!("Error when parsing command suggestion response: {}", e);
                    Err(e)
                },
            }
        },
        Err(e) => {
            println!("Error when getting response from api. Make sure you've added your secret key to your environment ('OPENAI_API_KEY=sk_....'): {}", e);
            Err(e)
        },
    };

    command_suggestion_response

    
}

// This is a function for handling a stream of messages from the open ai api, which uses EventSource to get server side events
pub async fn get_openai_response_stream(prompt: String, openai_api_key: String, options: cli::CommandLineOptions) -> Result<String, reqwest::Error>{
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));
    headers.append("Authorization", reqwest::header::HeaderValue::from_str(&format!("Bearer {}", openai_api_key)).unwrap());

    let body = OpenAiChatCompletionRequest {
        model: "gpt-4-1106-preview".to_string(),
        messages: vec![
            OpenAiMessageObject {
                role: "system".to_string(),
                content: "You are the personification of a user's command line interface. 
                You are helpful, friendly, and extremely knowledgable. 
                You exist in the context of an Arch Linux terminal emulator. 
                If the user is asking how to do something in the terminal, you will respond with a linux command line command to help them complete their task.
                When printing code, instead of listing the code language (i.e. ```bash, ```plaintext), number each code snipit starting from 1, (i.e. ```1, ```2).".to_string(),
            },
            OpenAiMessageObject {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        // Response format is a json object with the field 'type' and the value 'json_object'
        response_format: ResponseFormat{
            r#type: "text".to_string(),
        },
        stream: true
    };
    let mut res = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&body);

    let mut display_string = "".to_string();
    let mut es = EventSource::new(res);
    match es {
        Ok(mut es) => {
            while let Some(event) = es.next().await{
                
                match event {
                    Ok(Event::Open) => println!("Connection open!"),
                    Ok(Event::Message(message)) => {
                        // println!("Message: {}", message.data);
                        if message.data == "[DONE]" {
                            es.close();
                            break;
                        }
                        let message_data_to_json = serde_json::from_str::<OpenAiResponseByteStream>(&message.data);
                        match message_data_to_json {
                            Ok(js) => {
                                match &js.choices[0].delta.content {
                                    Some(content) => {
                                        
                                        // let content = content.replace("\n", "").to_string();
                                        display_string.push_str(&content);
                                        execute!(
                                            stdout(),
                                            cursor::MoveToColumn(0),
                                            Clear(crossterm::terminal::ClearType::All),
                                            Print(display_string.clone()),
                                        ).unwrap();
                                    },
                                    None => {},
                                }
                                
                            },
                            Err(e) => {
                                println!("Error when parsing object: {}", e);
                            },
                        }
                    },
                    Err(err) => {
                        println!("Error: {}", err);
                        es.close();
                    }
                }
        };
        },
        Err(e) => {
            println!("Error when creating event source: {}", e);
        },
    }
    println!();

    Ok("hey".to_string())
}