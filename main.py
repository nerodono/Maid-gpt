from maid.config import Config
from maid.ai.prompt_engine import PromptEngine
from maid.ai.engine import Ai
from maid.ai.schemas import Prompt, CompletionError, FinishReason
from maid.misc.typing import typing_status

from pathlib import Path
from aiogram import executor, Dispatcher, Bot, types
from sys import executable

from asyncio import subprocess


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
dp = Dispatcher(bot)


def try_decode(b: bytes) -> str:
    try:
        return b.decode()
    except UnicodeDecodeError:
        return repr(b)


async def deny(message: types.Message):
    await message.reply("Нет")


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
        f"Prompt ({total} tokens which costs {(total / 1000.0) * 0.002:.4f}): {message.text}"
    )
    choice = result.choices[0]
    content = choice.message.content

    if choice.finish_reason == FinishReason.content_filter:
        note = "<Ответ был сгенерирован под влиянием контент фильтра OpenAI>\n"
    modes = ["Markdown", ""]

    (before, after) = content.split(":", maxsplit=1)
    match before:
        case "python":
            if not ai.is_master(message.from_user):
                return await deny(message)

            proc = await subprocess.create_subprocess_exec(
                executable,
                "-c",
                after.lstrip(),
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            stdout, stderr = await proc.communicate()
            if stderr:
                answer_text = "Произошла ошибка:\n" + try_decode(stderr)
            else:
                answer_text = try_decode(stdout)
        case "plain":
            answer_text = after
        case _:
            answer_text = f"<Undefined action type {before}>\n{after}"

    for mode in modes:
        try:
            await message.reply(note + answer_text, parse_mode=mode)
        except Exception:
            continue
        break


executor.start_polling(dp, skip_updates=True)
