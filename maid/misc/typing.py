from aiogram.types import Message
from asyncio import create_task, sleep


def typing_status(handler):
    async def inner(message: Message):
        async def typer():
            while not done:
                await message.answer_chat_action("typing")
                await sleep(5)

        def reset_done():
            nonlocal done
            done = True

        task = create_task(handler(message, lambda: create_task(typer())))
        done = False

        task.add_done_callback(lambda _: reset_done())

    return inner
