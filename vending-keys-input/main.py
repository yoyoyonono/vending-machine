from pad4pi import rpi_gpio

factory = rpi_gpio.KeypadFactory()

factory.create_4_by_4_keypad()

def printKey(key):
    print(key)