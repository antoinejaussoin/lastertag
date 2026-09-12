# Family e-ink frame

A 13.3″ Spectra 6 panel in a picture frame. A **Pimoroni Pico Plus 2 W**
wakes once an hour, downloads a packed image, and sleeps. A **Rust server**
on the LAN builds that image from HTML/CSS plus the family calendar and
lists.

This folder is separate from the laser-tag prototype. Hardware to buy is
in [`shopping.md`](shopping.md), not [`docs/shopping-list.md`](../../docs/shopping-list.md).

## What you get

| Piece | Where |
|---|---|
| Hardware shopping list | [`shopping.md`](shopping.md) |
| Wiring / stack | [`wiring.svg`](wiring.svg), [`connections.svg`](connections.svg) |
| Pico protocol | [`firmware/PROTOCOL.md`](firmware/PROTOCOL.md) |
| Rust server + layout simulator | [`server/`](server/) |

## Layout workflow

1. Edit [`server/templates/dashboard.html`](server/templates/dashboard.html) and
   [`server/static/dashboard.css`](server/static/dashboard.css).
2. Open `/preview` in a browser. The iframe is the real 1600×1200 panel.
3. On the LAN, Chromium screenshots `/dashboard`, the server dithers to
   Spectra 6, and the Pico GETs `/frame.bin`.
4. If the family data has not changed, the checksum matches and the Pico
   does **not** refresh the glass.

The dashboard HTML must not include a ticking clock. A changing “updated at”
would make every hour look like a new image.

## Run the server

Chrome or Chromium is required only for `/frame.bin` / `/frame.png`. The
HTML simulator works without it.

```bash
cd projects/eink-frame/server
cp config.example.toml config.toml   # optional; demo data is the default
cargo run -- serve
```

Then open <http://127.0.0.1:8765/preview>.

Pretend to be the Pico:

```bash
cargo run -- pico-sim --url http://127.0.0.1:8765 --interval-secs 5
```

## Family iCloud calendar and lists

Apple does not offer a public “Family Sharing API”. What works:

1. Create an **app-specific password** at [account.apple.com](https://account.apple.com).
2. Put the Apple ID and that password in `config.toml`.
3. Set `calendars = ["Family"]` (or whatever the shared calendar is called
   in Calendar.app). Family Sharing calendars show up over CalDAV.
4. `todo_list` and `shopping_list` are separate CalDAV reminder lists.
   **Shopping is not the same list as to-dos.**

If Reminders no longer appear over CalDAV on your account (Apple has been
retiring that path), keep using:

- [`server/fixtures/shopping.json`](server/fixtures/shopping.json)
- [`server/fixtures/todos.json`](server/fixtures/todos.json)

or publish a read-only webcal URL in `sources.ics_urls`.

Never commit `config.toml` — it is gitignored.

## Pico side

The server is ready. The first panel bring-up should reuse the community
Inky 13.3 + Plus 2 W driver (see the protocol doc). The Pico only needs
Wi-Fi, an HTTP GET, and `el133_show_frame()` when the checksum changes.
