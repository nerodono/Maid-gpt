from maid.config import Config
from maid.maid import Maid
from maid.prompt import (ResultType, Prompt,
                         LayerPrompt, HouseRights)
from aiogram import types, Bot, Dispatcher, executor
from asyncio import sleep, subprocess

from sys import executable

config = Config.load()
ai = Maid(config)

bot = Bot(token=config.bot.token)
dp = Dispatcher(bot)


@dp.message_handler()
async def message_handler(message: types.Message):
    prompts = ai.responds_to(message)
    if prompts is None:
        return

    answered = False

    async def typer():
        while not answered:
            await message.answer_chat_action("typing")
            await sleep(4)

    ai._loop.create_task(typer())

    try:
        response = await ai.feed(prompts)
        match response.type_:
            case ResultType.bash_command:
                return await message.reply(
                    "Меня попросили выполнить баш команду")
            case ResultType.python_command:
                print(f"Executing {response.content}")
                result = await subprocess.create_subprocess_exec(
                    executable,
                    "-c",
                    response.content,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                )
                stdout, stderr = await result.communicate()

                if len(stdout) == 0:
                    if stderr:
                        stdout = b"Error: " + stderr
                    else:
                        stdout = b"<empty result>"

                if len(stdout) < 2000:
                    resp = await ai.feed([
                        Prompt.read_system("python_prelude",
                                           map=lambda s: s.format(
                                                   result=stdout.decode())),
                        LayerPrompt(
                            rights=HouseRights.master,
                            message="Каков был результат?\
                                    Не описывай то, что было выведено,\
                                    просто отправь конкретный результат",
                            tg_id=message.from_user.username,
                            name=message.from_user.full_name
                        ).to_prompt()
                    ])
                else:
                    resp = stdout.decode()
                return await message.reply(
                    f"<Выполнено: {response.content}>\n{resp.content}")
            case ResultType.plain:
                return await message.reply(response.content)
    finally:
        answered = True


executor.start_polling(dp, skip_updates=True)
