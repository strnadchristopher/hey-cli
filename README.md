# Hey-cli

Get Chat-GPT responses in your terminal.

## Chat-GPT in your terminal

Do you ever find yourself forgetting common linux commands? Do you ever wish you could just ask your terminal how to do something? Well now you can! With hey-cli, you can get responses from the chat-gpt model in your terminal. Just type in a prompt and get a response! If it comes back with any commands, you can run them quickly by typing "Y" when it asks you if you want to run the command. Or if it suggests multiple, you can type in the number of the command you want to run.

## Installation

```bash
git clone https://github.com/strnadchristopher/hey-cli
cd hey-cli
cargo build --release
```

Add the binary created in target/release/ to your path however you would like to do so. I personally use `~/.local/bin` and add that to my path.

- Note that after doing this, you'll be able to run the program with the command 'hey'
## Adding Your OpenAI API Key
In order for this bot to work, you must supply an open ai api key. Add that key to your environment variables as `OPENAI_API_KEY`. You can do this by adding the following to your `.bashrc` or `.zshrc` or whatever shell you use.

```bash
OPENAI_API_KEY=yourkeyhere
```

## Usage

```bash
hey [options] [prompt]
```

Options:
- `-h` or `--help`: Prints help information

Prompt:
- The prompt to use for the chat-gpt model. You can type in natural language and you do NOT have to use quotation marks, although it seems right now question marks will not work if you chose to omit quotation marks.


