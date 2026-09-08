# Web application

The web application will run on a small local server and provide:

- match setup, player/team assignment, and device status;
- a live scoreboard and per-player statistics;
- an operator interface for starting, pausing, and ending games;
- persistent players, matches, hits, and summary statistics;
- a device-facing API and real-time browser updates;
- export and backup of match history.

The initial deployment target is a Raspberry Pi on the same Wi-Fi network as
the Pico 2 W devices. A laptop can be used during development. The planned
default database is SQLite; internet hosting can be added later without making
the game dependent on an internet connection.
