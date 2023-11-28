// This is a simple rust program that calls the open ai chat gpt api
use reqwest;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct OpenAiMessageObject{
    role: String,
    content: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct OpenAiChatCompletionResponseUsage{
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct OpenAiChatCompletionResponseChoice {
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
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ResponseFormat{
    r#type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CommandSuggestionResponse{
    message: String,
    command_suggestion: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CommandLineOptions{
    help: bool,
    debug: bool,
    chat_input: String,
}

#[tokio::main]
async fn main() {
    // First, get the api key from the environment variable OPENAI_API_KEY
    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();
    // let openai_api_key = "ass".to_string();

    // Second, get arguments from the command line, can be any number of words, all compiled into one string
    let args: Vec<String> = std::env::args().collect();

    // Next, check if there are any arguments
    if args.len() < 2 {
        println!("Please enter a prompt");
        return;
    }
    let options = parse_command_line_arguments(args.clone());
    if options.help {
        println!("Usage: hey [options] [prompt]");
        println!("Options:");
        println!("  -h, --help     Prints help information");
        println!("  -d, --debug    Prints debug information");
        return;
    }
    // Next, get all the arguments and compile them into one string
    let prompt = options.chat_input.trim().to_string();
    // Next, print the prompt
    if options.debug{
        println!("Prompt: {}", prompt.clone());
    }

    // Next, call the openai api
    let command_suggestion_response = match get_response_object(prompt, openai_api_key, options).await{
        Ok(res) => {res},
        Err(e) => {
            println!("Error when calling the OpenAi API: {}", e);
            return;
        },
    };
    
    // Next, print the response
    println!("{}", command_suggestion_response.message);

    if command_suggestion_response.command_suggestion == "NULL" {
        return;
    }

    println!("Would you like to run the command: {} [Y/n]", command_suggestion_response.command_suggestion);
    // Wait for keyboard input, and then read it
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input){
        Ok(_) => {// If the user wants to run the command, run it
            if input.trim() == "Y" || input.trim() == "y" {
                // Run the command
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command_suggestion_response.command_suggestion)
                    .output()
                    .expect("failed to execute process");
                // Print the output
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }},
        Err(e) => {
            println!("Error reading command line input: {}", e);
            return;
        },
    }
}

fn parse_command_line_arguments(args: Vec<String>) -> CommandLineOptions {
    let mut options = CommandLineOptions{
        help: false,
        debug: false,
        chat_input: "".to_string(),
    };

    // remove the first argument, which is the program name
    let args = &args[1..];

    for arg in args {
        match arg.as_str() {
            "--debug" | "-d" => {
                options.debug = true;
            },
            "--help" | "-h" => {
                options.help = true;
            },
            _ => {
                options.chat_input = options.chat_input + " " + &arg;
            },
        }
    }

    options
}

async fn get_response_object(prompt: String, openai_api_key: String, options: CommandLineOptions) -> Result<CommandSuggestionResponse, serde_json::Error> {
    let api_response = match get_openai_response(prompt, openai_api_key, options).await {
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

async fn get_openai_response(prompt: String, openai_api_key: String, options: CommandLineOptions) -> Result<String, reqwest::Error>{
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
        }
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
