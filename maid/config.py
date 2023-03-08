from __future__ import annotations

from dataclasses import dataclass
from dataclass_factory import Factory
from pytoml import load


@dataclass
class BotConfig:
    username: str
    token: str
    prefix: str


@dataclass
class AiConfig:
    token: str
    master: str


@dataclass
class Config:
    bot: BotConfig
    ai: AiConfig

    @classmethod
    def load(cls) -> Config:
        return Factory().load(load(open("config.toml")), cls)
