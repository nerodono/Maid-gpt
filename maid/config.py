from __future__ import annotations

from tomli import loads
from pathlib import Path
from dataclasses import dataclass
from dataclass_factory import Factory
from enum import Enum


class Sex(str, Enum):
    female = "female"
    male = "male"


@dataclass
class DanbooruConfig:
    token: str
    login: str


@dataclass
class MiscConfig:
    danbooru: DanbooruConfig


@dataclass
class CharacterConfig:
    name: str
    sex: Sex


@dataclass
class BotConfig:
    username: str
    token: str
    prefix: str


@dataclass
class AiMasterConfig:
    name: str
    username: str
    sex: Sex


@dataclass
class AiConfig:
    master: AiMasterConfig
    character: CharacterConfig

    model: str
    token: str


@dataclass
class Config:
    ai: AiConfig
    bot: BotConfig
    misc: MiscConfig

    @classmethod
    def read(cls, path: Path) -> Config:
        content = path.read_text()
        parsed = loads(content)

        return Factory().load(parsed, cls)
