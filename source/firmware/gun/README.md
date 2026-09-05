# Gun firmware

Planned responsibilities:

- read trigger, reload, and menu buttons;
- generate a 38 kHz modulated infrared shot packet;
- enforce ammunition, reload time, fire rate, and game state;
- show health, ammunition, score, and connection state on the OLED;
- exchange events and state with the game server over Wi-Fi;
- keep a short local event queue during temporary network loss.

The gun firmware must never drive the infrared LED directly from a GPIO pin.
It will control the LED through the transistor driver documented in the
hardware design.
