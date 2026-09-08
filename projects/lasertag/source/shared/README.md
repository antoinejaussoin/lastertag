# Shared protocol

This directory will define the versioned contracts used by firmware and the
server:

- player, team, device, match, and receiver-zone identifiers;
- infrared shot packet fields and checksum;
- network event schemas;
- sequence numbers and duplicate-event rules;
- protocol compatibility and test vectors.

The infrared packet should contain only the data needed to validate a shot,
such as protocol version, shooter/team ID, shot sequence, and checksum. The
server remains authoritative for scoring and persistence.
