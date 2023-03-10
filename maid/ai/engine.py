import re
from typing import List, Optional

from .prompt_engine import PromptEngine
from .openai import OpenAiChat
from .schemas import Prompt, CompletionResult, CompletionError
from asyncio import sleep

from ..config import Config
from ..misc.role import HouseRole

from aiogram.types import Message, User


class Ai:
    def __init__(
        self,
        config: Config,
        engine: PromptEngine,
        core_prompt: Prompt,
        max_retries: int = 5,
        sleep_time: float = 5.0,
    ) -> None:
        self._chat = OpenAiChat(
            token=config.ai.token, model=config.ai.model
        )
        self._prefix = re.compile(config.bot.prefix, flags=re.IGNORECASE)
        self._engine = engine
        self._config = config

        self.max_retries = max_retries
        self.sleep_time = sleep_time

        self.core = core_prompt

    async def answer_to(
        self,
        prompts: List[Prompt],
        temperature: float = 0.2,
        tries: int = 0,
    ) -> CompletionResult | CompletionError:
        result = await self._chat.complete(
            prompts, temperature=temperature
        )
        if isinstance(result, CompletionError):
            print(
                "Completion error:",
                result.error,
                f". Retrying in {self.sleep_time}secs",
            )

            if tries >= self.max_retries:
                return result

            await sleep(self.sleep_time)
            return await self.answer_to(prompts, temperature, tries + 1)

        return result

    def is_master(self, user: User) -> bool:
        return self.detect_role_of(user) == HouseRole.master

    def responds_to(self, message: Message) -> Optional[List[Prompt]]:
        prefix_match = self._prefix.match(message.text)
        prepended_prompts = []

        prefix_required = True
        if message.reply_to_message:
            replied = message.reply_to_message
            cfg = self._config

            if replied.from_user.username == cfg.bot.username:
                prefix_required = False
                prepended_prompts.append(Prompt.assistant(replied.text))

        if message.from_user.id == message.chat.id:
            prefix_required = False
            prepended_prompts.append(
                Prompt.system(
                    "You and your interlocutor in the private conversation right now"
                )
            )

        text = message.text
        if not prefix_match and prefix_required:
            return
        elif prefix_match:
            end = prefix_match.span()[1]
            text = text[end:].strip()

        user = message.from_user
        role = self.detect_role_of(user)

        core_rendered = self.core.render(
            role=role, user=user, message=message, config=self._config
        )

        return [
            *prepended_prompts,
            *[
                prompt.render(
                    role=role,
                    user=user,
                    message=message,
                    config=self._config,
                )
                for prompt in self._engine.prompts
            ],
            core_rendered,
            Prompt.user(text or "*Silence*"),
        ]

    def detect_role_of(self, user: User) -> HouseRole:
        if user.username == self._config.ai.master.username:
            return HouseRole.master

        return HouseRole.guest
