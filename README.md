# Hi-cli

Get Chat-GPT responses in your terminal.

## Wow so cool another Chat-Gpt thing you're so original

I know thank you. I personally find this useful because I'm always forgetting common or uncommon linux commands so I made this in a couple hours to help me out. I hope it helps you too. Written in rust.

## Installation

```bash
git clone https://github.com/strnadchristopher/hey-cli
cd hey-cli
cargo build --release
```

Add that binary to your path however you would like to do so. I personally use `~/.local/bin` and add that to my path.

## Adding Your OpenAI API Key
In order for this bot to work, you must get a open ai api key. Add that key to your environment variables as `OPENAI_API_KEY`. You can do this by adding the following to your `.bashrc` or `.zshrc` or whatever shell you use.

```bash
OPENAI_API_KEY=yourkeyhere
```

## Usage

```bash
hey-cli [options] [prompt]
```

Options:
- `-h` or `--help`: Prints help information
- `-d` or `--debug`: Prints debug information when running

Prompt:
- The prompt to use for the chat-gpt model. You can type in natural language and you do NOT have to use quotation marks, although it seems right now question marks will not work if you chose to omit quotation marks.



