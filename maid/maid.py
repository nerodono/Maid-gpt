import re

from .config import Config
from .prompt_engine import PromptEngine
from .prompt import Prompt, LayerPrompt, HouseRights, LayerResult, ResultType

from aiogram.types import Message, User
from typing import Optional, List

from asyncio import sleep

from json import loads
from asyncio import get_event_loop

from json.decoder import JSONDecodeError

from openai.error import RateLimitError
import openai


class Maid:
    def __init__(self, config: Config) -> None:
        self._config = config
        self._prefix = re.compile(config.bot.prefix)
        self._loop = get_event_loop()

        openai.api_key = config.ai.token
        self.engine = PromptEngine(
            initial_prompts=[
                Prompt.read_system("format"),
                Prompt.read_system("base_prompt", map=lambda x: x.format(
                    name=config.ai.name,
                    sex=config.ai.sex,
                    master_name=config.ai.master.name,
                    master_tgid=config.ai.master.username,
                    master_note=config.ai.master.note,
                )),
                Prompt.read_system("guest_database"),
            ]
        )

    async def feed(self, prompts: List[Prompt]) -> LayerResult:
        try:
            result: dict = await self._loop.run_in_executor(
                None,
                lambda: openai.ChatCompletion.create(
                    model="gpt-3.5-turbo",
                    messages=self.engine.with_prompts(*prompts).to_dict()
                ),
            )
        except RateLimitError as e:
            await sleep(5)
            print("Rate limit {}".format(e))
            return await self.feed(prompts)

        for response in result["choices"]:
            try:
                response = loads(response["message"]["content"])
            except JSONDecodeError:
                continue
            return LayerResult.from_json(response)

        return LayerResult(
            type_=ResultType.plain,
            content=result["choices"][0]["message"]["content"]
        )

    def infer_rights(self, of: User) -> HouseRights:
        if of.username == self._config.ai.master.username:
            return HouseRights.master
        return HouseRights.guest

    def create_prompt(self, message: Message, with_text: str) -> LayerPrompt:
        return LayerPrompt(
            rights=self.infer_rights(message.from_user),
            message=with_text,
            tg_id=message.from_user.username,
            name=message.from_user.full_name,
        )

    def responds_to(self, message: Message) -> Optional[List[Prompt]]:
        is_direct_messages = message.chat.id == message.from_user.id
        if message.reply_to_message is not None:
            reply = message.reply_to_message

            if reply.from_user.username == self._config.bot.username:
                return [
                    Prompt.assistant(reply.text),
                    self.create_prompt(message, message.text).to_prompt(),
                ]

        prefix_match = self._prefix.match(message.text.lower())
        if not prefix_match and not is_direct_messages:
            return

        if prefix_match:
            span = prefix_match.span()[1]
            cut = message.text[span:].strip()
        else:
            cut = message.text

        return [self.create_prompt(message, cut).to_prompt()]
