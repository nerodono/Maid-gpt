from typing import List
from aiohttp import ClientSession
from dataclass_factory import Factory
from asyncio import create_task

from .schemas import (
    CompletionResult,
    Prompt,
    CompletionTokens,
    CompletionChoice,
    CompletionError,
)


class OpenAiChat:
    def __init__(self, token: str, model: str = "gpt-3.5-turbo") -> None:
        self._session = ClientSession(base_url="https://api.openai.com")
        self._factory = Factory()

        self.token = token
        self.model = model

    async def complete(
        self, prompts: List[Prompt], temperature: float = 0.7
    ) -> CompletionResult | CompletionError:
        async with self._session.post(
            "/v1/chat/completions",
            json={
                "model": self.model,
                "messages": self._factory.dump(prompts, List[Prompt]),
                "temperature": temperature,
            },
            headers={"Authorization": f"Bearer {self.token}"},
        ) as response:
            json = await response.json()

            if "choices" not in json:
                return CompletionError(error=json["error"]["message"])

            usage = json["usage"]
            return CompletionResult(
                usage=CompletionTokens(
                    prompt=usage["prompt_tokens"],
                    completion=usage["completion_tokens"],
                    total=usage["total_tokens"],
                ),
                choices=self._factory.load(
                    json["choices"], List[CompletionChoice]
                ),
            )

    def __del__(self):
        create_task(self._session.close())
