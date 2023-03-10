from typing import List
from aiohttp import ClientSession
from ..config import DanbooruConfig
from base64 import b64encode

from dataclasses import dataclass


@dataclass
class DanPost:
    tag_string: str
    file_url: str


class Danbooru:
    def __init__(self, cfg: DanbooruConfig) -> None:
        self._config = cfg
        self._session = ClientSession(
            base_url="https://danbooru.donmai.us"
        )

    @property
    def authorization_header(self) -> str:
        pair = f"{self._config.login}:{self._config.token}".encode()
        return f"Basic {b64encode(pair).decode()}"

    async def search_posts(self, tags: List[str]) -> List[DanPost]:
        async with self._session.get(
            "/posts.json",
            params={
                "random": "",
                "tags": " ".join(tags),
            },
        ) as response:
            json = await response.json()
            return [
                DanPost(
                    tag_string=n["tag_string"],
                    file_url=(n.get("large_file_url") or "<No url>"),
                )
                for n in json
            ]

    async def suggest(self, query: str) -> List[str]:
        async with self._session.get(
            "/tags.json",
            params={"search[name_matches]": f"*{query}*"},
            headers={"Authorization": self.authorization_header},
        ) as response:
            return [tag_js["name"] for tag_js in await response.json()]
