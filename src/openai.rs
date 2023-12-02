// This is a simple rust program that calls the open ai chat gpt api
use reqwest;
use reqwest_eventsource::EventSource;

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
use std::io::stdout;

use crossterm::{
    cursor,
    execute,
    terminal::Clear,
    style::Print,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct StreamCommandSuggestionResponse{
    pub message: String,
    pub command_suggestion: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CommandSuggestion{
    pub command: String,
}

use regex;

pub async fn get_stream_response_object(prompt: String, openai_api_key: String) -> Result<Vec<CommandSuggestion>, serde_json::Error> {
    let api_response = match generate_response(prompt, openai_api_key).await {
        Ok(res) => {res},
        Err(e) => {
            println!("Error: {}", e);
            "{}".to_string()
        },
    };

    // Parse the command suggestions from response

    // First, create an empty vector of CommandSuggestion structs
    let mut command_suggestions = Vec::<CommandSuggestion>::new();

    // The api_response may have instances of ```1, ```2, etc, which are used to number code snipits, each code snippit ends with trailing ```
    // We need to extract the code snippits from the api_response, and then remove the ```1, ```2, etc, and then add the code snippits to the command_suggestions vector
    
    // First, use regex to find every instance of ```(any series of chracters)``` and then add each instance to a vector
    let mut code_snippits = Vec::<String>::new();
    let re = regex::Regex::new(r"```(.|\n)*?```").unwrap();
    for cap in re.captures_iter(&api_response) {
        // We're going to add each code snippit to the vector
        // But first we must remove the ``` characters as well as any numbers and any newlines characters
        code_snippits.push(
            cap[0].to_string()
            .replace("```", "")
            .replace("\n", "")
            .replace("1", "")
            .replace("2", "")
            .replace("3", "")
            .replace("4", "")
            .replace("5", "")
            .replace("6", "")
            .replace("7", "")
            .replace("8", "")
            .replace("9", "")
            .replace("0", "")
        );
        command_suggestions.push(CommandSuggestion { command: (
            cap[0].to_string()
            .replace("```", "")
            .replace("\n", "")
            .replace("1", "")
            .replace("2", "")
            .replace("3", "")
            .replace("4", "")
            .replace("5", "")
            .replace("6", "")
            .replace("7", "")
            .replace("8", "")
            .replace("9", "")
            .replace("0", "")
        ) }
        );
    }
    if code_snippits.len() > 0{
        println!("Code snippits: {:?}", code_snippits);
    }

    Ok(command_suggestions)
}

// This is a function for handling a stream of messages from the open ai api, which uses EventSource to get server side events
pub async fn generate_response(prompt: String, openai_api_key: String) -> Result<String, reqwest::Error>{
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
    let res = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&body);

    let mut response_string = "".to_string();
    let es = EventSource::new(res);
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
                                        response_string.push_str(&content);
                                        execute!(
                                            stdout(),
                                            cursor::MoveToColumn(0),
                                            Clear(crossterm::terminal::ClearType::All),
                                            Print(response_string.clone()),
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

    Ok(response_string)
}