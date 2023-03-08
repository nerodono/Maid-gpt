from maid.config import Config
from maid.maid import Maid
from maid.prompt import (ResultType, Prompt,
                         LayerPrompt, HouseRights)
from aiogram import types, Bot, Dispatcher, executor
from asyncio import sleep

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
                result = eval(response.content)
                resp = await ai.feed([
                    Prompt.read_system("python_prelude",
                                       map=lambda s: s.format(
                                            result=str(result))),
                    LayerPrompt(
                        rights=HouseRights.master,
                        message="Каков результат? Ответь что конкретно\
                                 получилось",
                        tg_id=message.from_user.username,
                        name=message.from_user.full_name
                    ).to_prompt()
                ])
                return await message.reply(
                    f"(Выполнено: {response.content}) {resp.content}")
            case ResultType.plain:
                return await message.reply(response.content)
    finally:
        answered = True


executor.start_polling(dp, skip_updates=True)
