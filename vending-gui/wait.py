import asyncio
import json
import pathlib
import websockets


async def handle_socket(url: str):
    async for ws in websockets.connect(url):
        message = json.loads(await ws.recv())
        print(message)
        if "paymentSuccess" in json.loads(message['transactionStatus']):
            return
        
asyncio.get_event_loop().run_until_complete(
    handle_socket(pathlib.Path('url.txt').read_text()))