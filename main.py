from maid.config import Config
from maid.ai.prompt_engine import PromptEngine
from maid.ai.engine import Ai
from maid.ai.schemas import Prompt, CompletionError, FinishReason
from maid.misc.typing import typing_status

from pathlib import Path
from aiogram import executor, Dispatcher, Bot, types
from sys import executable

from random import choice as random_choice

from asyncio import subprocess

from maid.misc.danbooru import Danbooru


def read_prompt(prompt_name: str, **fmt_args) -> Prompt:
    actual_path = Path("assets/prompts") / f"{prompt_name}.txt"
    print(f"Loading {actual_path} prompt...")

    p = actual_path.read_text()
    if fmt_args:
        p = p.format(**fmt_args)
    return Prompt.system(p)


config = Config.read(Path("config.toml"))
ai = Ai(
    config=config,
    core_prompt=read_prompt("core"),
    engine=PromptEngine(
        prepended=[
            read_prompt(
                "base_prompt",
                sex=config.ai.character.sex,
                master_name=config.ai.master.name,
                master_sex=config.ai.master.sex,
                name=config.ai.character.name,
            ),
            read_prompt("abilities"),
            read_prompt("guest_database"),
        ]
    ),
)

bot = Bot(token=config.bot.token)
danbooru = Danbooru(config.misc.danbooru)
dp = Dispatcher(bot)


def try_decode(b: bytes) -> str:
    try:
        return b.decode()
    except UnicodeDecodeError:
        return repr(b)


async def deny(message: types.Message):
    await message.reply("<GPT-3 Access leak> Нет")


@dp.message_handler()
@typing_status
async def message_handler(message: types.Message, typer: callable):
    prompts = ai.responds_to(message)
    if not prompts:
        return

    typer()

    result = await ai.answer_to(prompts)
    if isinstance(result, CompletionError):
        await message.reply(
            f"Не получилось ответить, ошибка: {result.error}"
        )
        return

    note = ""
    total = result.usage.total
    print(
        f"Prompt ({total} tokens which costs {(total / 1000.0) * 0.002:.4f}) from {message.from_user}: {message.text}"
    )
    choice = result.choices[0]
    content = choice.message.content

    if choice.finish_reason == FinishReason.content_filter:
        note = "<Ответ был сгенерирован под влиянием контент фильтра OpenAI>\n"
    modes = ["Markdown", ""]

    try:
        (before, after) = content.split(":", maxsplit=1)
    except ValueError:
        before = "plain"
        after = content

    match before:
        case "danbooru_search":
            tags = [each.strip() for each in after.split(",")]
            posts = await danbooru.search_posts(tags)
            if not posts:
                prompts = ai.responds_to(message, suggestion=True)
                answer = await ai.answer_to(
                    [
                        *prompts,
                        Prompt.system(
                            "You tried to find danbooru pictures with the following tags: "
                            + f"{', '.join(tags)}. But you haven't found anything. "
                            "When I say <provide> you must provide me answer for that situation. "
                            "Use russian language in your answer",
                        ),
                        Prompt.user("<provide>"),
                    ]
                )
                return await message.reply(
                    answer.choices[0].message.content
                )
            post = random_choice(posts)
            answer_text = f"Search tags: {tags}, [{post.tag_string}]({post.file_url})"
        case "danbooru_suggest":
            tags = await danbooru.suggest(after)
            prompts = ai.responds_to(message, suggestion=True)
            answer = await ai.answer_to(
                [
                    *prompts,
                    Prompt.system(
                        "You were asked to suggest tags for the user "
                        "you have list of the following tags:"
                        + f" {', '.join(tags)}."
                        + " When I say <provide> You must provide full "
                        "answer mentioning all tags from the list. "
                        "Use Markdown in your answer, wrap every tag name in the code block "
                        " Sort them by the relevance, use Russian language in your answer. "
                        "Your answer type must be plain"
                        + f"Also the query was {after}"
                    ),
                    Prompt.user("<provide>"),
                ]
            )

            answer_text = answer.choices[0].message.content
        case "python":
            if not ai.is_master(message.from_user):
                return await deny(message)

            code = after.lstrip()
            if code.startswith("```"):
                code = code.removeprefix("```")
                code = code.removeprefix("\n")
            proc = await subprocess.create_subprocess_exec(
                executable,
                "-c",
                code,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            stdout, stderr = await proc.communicate()
            prepend = f"<executed {code}>"
            if stderr:
                answer_text = "Произошла ошибка:\n" + try_decode(stderr)
            else:
                answer_text = try_decode(stdout)

            answer_text = prepend + "\n" + answer_text
        case "plain":
            answer_text = after
        case _:
            answer_text = f"{before}{after}"

    for mode in modes:
        try:
            await message.reply(note + answer_text, parse_mode=mode)
        except Exception:
            continue
        break


executor.start_polling(dp, skip_updates=True)
