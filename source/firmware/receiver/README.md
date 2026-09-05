# Receiver-hub firmware

One wearable Raspberry Pi Pico W serves all receiver zones for a player.

Planned responsibilities:

- sample four or more 38 kHz demodulating infrared receivers;
- identify the hit zone and decode the complete shot packet;
- reject malformed, duplicate, self, and out-of-game hits;
- report validated hit events to the game server over Wi-Fi;
- provide local acknowledgement even if the server response is delayed;
- buffer events briefly through transient network interruptions.

Receiver inputs must use local supply decoupling and should use interrupt-capable
GPIOs. The exact pin map will be fixed after the breadboard prototype.
