# Maid-GPT

Stupid and perverted maid bot for the Telegram.

# Abilities

- Can answer you in the natural language
- Can execute python code on the running machine (but only master can do this)
- Customizable prompts

# Setup guide

- Get your API token from the https://platform.openai.com/account/api-keys
- Create bot account through the https://t.me/BotFather telegram bot

- Fill all necessary fields in the `config.toml` file (example configuration can be found at `config.toml.example`)

- Download and install python interpreter (3.10+) from https://python.org or using your linux distribution's package manager
- install poetry using the following command:
```
$ python -m pip install --user poetry
```
- Install dependencies by running
```
$ poetry install
```
In the project dir

- Finally start the bot:
```
$ poetry run python main.py
```
