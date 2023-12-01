#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CommandLineOptions{
    pub help: bool,
    pub debug: bool,
    pub chat_input: String,
}

pub fn parse_command_line_arguments(args: Vec<String>) -> CommandLineOptions {
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