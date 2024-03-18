from pad4pi import rpi_gpio
from pynput.keyboard import Key, Controller

keyboard = Controller()

KEYPAD = [
    ["1", "2", "3", "A"],
    ["4", "5", "6", "B"],
    ["7", "8", "9", "C"],
    ["*", "0", "#", "D"]
]

ROW_PINS = [4, 14, 15, 17] # BCM numbering
COL_PINS = [18, 27, 22, 23] # BCM numbering

factory = rpi_gpio.KeypadFactory()

keypad = factory.create_keypad(keypad=KEYPAD, row_pins=ROW_PINS, col_pins=COL_PINS)

def printKey(key: str):
    if key == '*' or key == '#':
        keyboard.press(Key.enter)
        keyboard.release(Key.enter)
    else:
        keyboard.press(key.lower())
        keyboard.release(key.lower())

keypad.registerKeyPressHandler(printKey)

while True:
    pass
