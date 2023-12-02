mod cli;
mod openai;

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

    let options = cli::parse_command_line_arguments(args.clone());

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

    
    let command_suggestions = match openai::get_stream_response_object(prompt, openai_api_key).await{
        Ok(res) => {res},
        Err(e) => {
            println!("Error when calling the OpenAi API: {}", e);
            return;
        },
    };

    

    match command_suggestions.len() {
        0 =>{
            return;
        }
        1 =>{
            let command_string = command_suggestions[0].command.clone();
            println!("Would you like to run the command: {} [Y/n]", command_suggestions[0].command);

            let mut input = String::new();
            
            match std::io::stdin().read_line(&mut input){
                Ok(_) => {// If the user wants to run the command, run it
                    if input.trim() == "Y" || input.trim() == "y" {
                        // Run the command
                        let output = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(command_string)
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
        },
        _ =>{
            println!("Which command would you like to run? [1-{}]", command_suggestions.len());
            
            let mut input = String::new();
            
            match std::io::stdin().read_line(&mut input){
                Ok(_) => {// If the user wants to run the command, run it
                    let command_index = match input.trim().parse::<usize>(){
                        Ok(res) => {res},
                        Err(_) => {
                            println!("Ok, no commands will be run.");
                            return;
                        },
                    };
                    if command_index > command_suggestions.len() || command_index == 0 {
                        println!("Ok, no commands will be run.");
                        return;
                    }
                    let command_string = command_suggestions[command_index - 1].command.clone();
                    // Run the command
                    let output = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(command_string)
                        .output()
                        .expect("failed to execute process");
                    // Print the output
                    println!("{}", String::from_utf8_lossy(&output.stdout));

                    },
                Err(e) => {
                    println!("Error reading command line input: {}", e);
                    return;
                },
            }
        }
    }

    

}



