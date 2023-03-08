from __future__ import annotations

from enum import Enum
from dataclasses import dataclass
from os.path import join
from json import dumps

from ._factory import GLOBAL_FACTORY


class PromptType(str, Enum):
    system_instruction = "system"
    user_input = "user"
    assistent_answer = "assistant"


class ResultType(str, Enum):
    bash_command = "bash_command"
    python_command = "python_command"

    plain = "plain"


class HouseRights(str, Enum):
    master = "master"
    guest = "guest"


@dataclass
class LayerResult:
    type_: ResultType
    content: str

    @classmethod
    def from_json(cls, json: dict) -> LayerResult:
        return LayerResult(type_=ResultType(json['type_']),
                           content=json['content'])


@dataclass
class Prompt:
    role: PromptType
    content: str

    def to_dict(self) -> dict:
        return GLOBAL_FACTORY.dump(self)

    @classmethod
    def system(cls, content: str) -> Prompt:
        return Prompt(role=PromptType.system_instruction, content=content)

    @classmethod
    def user(cls, content: str) -> Prompt:
        return Prompt(role=PromptType.user_input, content=content)

    @classmethod
    def assistant(cls, content: str) -> Prompt:
        return Prompt(role=PromptType.assistent_answer, content=content)

    @classmethod
    def read_system(cls, file: str,
                    role=PromptType.system_instruction,
                    map=lambda x: x) -> Prompt:
        return cls(
            role=role,
            content=map(open(join("assets", file + ".txt")).read()),
        )


@dataclass
class LayerPrompt:
    rights: HouseRights
    message: str

    tg_id: str
    name: str

    def to_dict(self) -> dict:
        return GLOBAL_FACTORY.dump(self)

    def to_prompt(self) -> Prompt:
        return Prompt(
            role=PromptType.user_input,
            content=dumps(self.to_dict(), ensure_ascii=False))
