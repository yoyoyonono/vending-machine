from pad4pi import rpi_gpio
from pynput.keyboard import Key, Controller
from time import sleep

keyboard = Controller()

KEYPAD = [
    ["1", "2", "3", "a"],
    ["4", "5", "6", "b"],
    ["7", "8", "9", "c"],
    ["*", "0", "#", "d"]
]

ROW_PINS = [4, 14, 15, 17] # BCM numbering
COL_PINS = [18, 27, 22, 23] # BCM numbering

factory = rpi_gpio.KeypadFactory()

keypad = factory.create_keypad(keypad=KEYPAD, row_pins=ROW_PINS, col_pins=COL_PINS)

def printKey(key: str):
    print(repr(key))
    if key == '*' or key == '#':
        keyboard.press(Key.enter)
        sleep(0.1)
        keyboard.release(Key.enter)
    else:
        keyboard.press(key.lower())
        sleep(0.1)
        keyboard.release(key.lower())

keypad.registerKeyPressHandler(printKey)

while True:
    pass
