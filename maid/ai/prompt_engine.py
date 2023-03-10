from __future__ import annotations

from typing import List
from .schemas import Prompt


class PromptEngine:
    def __init__(self, prepended: List[Prompt]) -> None:
        self.prompts = prepended

    def with_prompts(self, *prompts) -> PromptEngine:
        return PromptEngine([*self.prompts, *prompts])
