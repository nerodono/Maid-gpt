from __future__ import annotations

from typing import List
from .prompt import Prompt


class PromptEngine:
    def __init__(self, initial_prompts: List[Prompt]) -> None:
        self._prompts = initial_prompts

    def with_prompts(self, *args) -> PromptEngine:
        return PromptEngine([*self._prompts, *args])

    def to_dict(self) -> List[dict]:
        return [prompt.to_dict() for prompt in self._prompts]
