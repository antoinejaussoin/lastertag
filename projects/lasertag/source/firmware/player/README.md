# Player firmware

One Raspberry Pi Pico 2 W controls all electronics for one player.

Planned responsibilities:

- read trigger, reload, and menu buttons;
- generate a 38 kHz modulated infrared shot packet;
- read four or more separately wired 38 kHz body receiver zones;
- validate and deduplicate hits while rejecting the player's own shots;
- enforce local ammunition, reload time, fire rate, and game state;
- show health, ammunition, score, and connection state on the OLED;
- exchange events and authoritative state with the game server over Wi-Fi;
- keep a short local event queue during temporary network loss.

Each receiver uses its own GPIO so firmware can preserve the hit zone. Receiver
cables and the gun controls converge on the same Pico 2 W, located either in the
gun or in a wearable enclosure. The IR LED must be driven through a transistor,
never directly from a GPIO pin.
