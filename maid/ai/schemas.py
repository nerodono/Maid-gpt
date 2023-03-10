from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import List, Optional

from mako.template import Template


@dataclass
class CompletionError:
    error: str


class Role(str, Enum):
    user = "user"
    assistant = "assistant"
    system = "system"


class FinishReason(str, Enum):
    stop = "stop"
    length = "length"
    content_filter = "content_filter"
    null = "null"


@dataclass
class CompletionTokens:
    prompt: int
    completion: int
    total: int


@dataclass
class CompletionMessage:
    role: Role
    content: str


@dataclass
class CompletionChoice:
    message: CompletionMessage
    finish_reason: Optional[FinishReason]
    index: int


@dataclass
class CompletionResult:
    usage: CompletionTokens
    choices: List[CompletionChoice]


@dataclass
class Prompt:
    role: Role
    content: str

    def join(self, with_: Prompt) -> Prompt:
        return Prompt(
            role=self.role, content=f"{self.content}.\n\n{with_.content}"
        )

    def render(self, *pos, **kws) -> Prompt:
        return Prompt(
            role=self.role,
            content=Template(self.content).render(*pos, **kws),
        )

    @classmethod
    def assistant(cls, content: str) -> Prompt:
        return Prompt(role=Role.assistant, content=content)

    @classmethod
    def system(cls, content: str) -> Prompt:
        return Prompt(role=Role.system, content=content)

    @classmethod
    def user(cls, content: str) -> Prompt:
        return Prompt(role=Role.user, content=content)
