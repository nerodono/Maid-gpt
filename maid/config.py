from __future__ import annotations

from dataclasses import dataclass
from dataclass_factory import Factory
from pytoml import load
from enum import Enum

class Sex(str, Enum):
    male = "male"
    female = "female"


@dataclass
class BotConfig:
    username: str
    token: str
    prefix: str


@dataclass
class AiConfig:
    token: str
    name: str
    sex: Sex

    master: MasterConfig

@dataclass
class MasterConfig:
    name: str
    username: str
    note: str


@dataclass
class Config:
    bot: BotConfig
    ai: AiConfig

    @classmethod
    def load(cls) -> Config:
        return Factory().load(load(open("config.toml")), cls)
