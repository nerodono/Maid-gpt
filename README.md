# Maid-LLaM

<img align="left" src="assets/art.jpg" width="250" />

Chat bot with answers powered by a large language models. Currently all natural language processing and answer generation is done through the OpenAI `gpt-3.5-turbo` model.

<br clear="left"/>

# Abilities

- **Answers in the natural language**: because of LLaMs usage bot can read your requests in free form and answer you like a human.
- **Highly customizable prompts**: Every prompt is a `askama` template, so you can tweak how prompt will be sent to the model exclusively for each message.
- **Custom abilities**: you can customize text abilities of your bot through the [abilities.txt](assets/prompts/abilities.txt) prompt.

# To-Do

- [ ] WebHook for Telegram
- [ ] WebHook for VKontakte
- [ ] Replace **teloxide** with own lightweight implementation for only our purposes
- [ ] Vkontakte platform
- [x] Telegram platform
