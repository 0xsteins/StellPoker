# Unified logging schema

StellPoker services should emit one JSON object per line so coordinator, MPC node, contract tooling, and dev scripts can be queried together in Loki/Grafana.

## Required fields

| Field | Type | Description |
| --- | --- | --- |
| `timestamp` | string | RFC3339/ISO timestamp generated at log emission time. |
| `level` | string | Lowercase or uppercase log level: `trace`, `debug`, `info`, `warn`, `error`. |
| `service` | string | Stable service name, for example `coordinator`, `mpc-node-0`, `mpc-node-1`, `contract-deployer`. |
| `session_id` | string or null | Poker/MPC session identifier when the log belongs to one game session. |
| `request_id` | string or null | HTTP, websocket, or job request correlation ID. |
| `duration_ms` | number or null | Duration for completed operations. Use `null` when the event is not timed. |

## Recommended fields

| Field | Type | Description |
| --- | --- | --- |
| `message` | string | Human-readable summary. |
| `target` | string | Rust tracing target/module path. |
| `table_id` | number or string | Poker table ID when available. |
| `player` | string | Stellar address or player handle when safe to log. |
| `contract_id` | string | Soroban contract ID for deploy/invoke events. |
| `network` | string | `local`, `testnet`, `mainnet`, or staging network label. |
| `error` | string | Sanitized error summary. Do not log secrets or private cards. |

## Example

```json
{
  "timestamp": "2026-09-24T12:00:00.000Z",
  "level": "info",
  "service": "coordinator",
  "session_id": "table-7-hand-42",
  "request_id": "req_01K5...",
  "duration_ms": 18,
  "message": "deal proof accepted",
  "table_id": 7,
  "contract_id": "CA3R...CHAV",
  "network": "testnet"
}
```

## Rust tracing guidance

Use structured fields instead of embedding key/value pairs inside the message:

```rust
tracing::info!(
    service = "coordinator",
    session_id = %session_id,
    request_id = %request_id,
    duration_ms = elapsed.as_millis() as u64,
    table_id,
    "deal proof accepted"
);
```

For MPC node logs, set `service` to the node identity (`mpc-node-0`, `mpc-node-1`, `mpc-node-2`) so dashboards can split quorum-level failures from a single-node failure.

## Safety rules

- Never log private cards, card shares, CRS material, secret keys, session encryption keys, or bearer/API tokens.
- Hash or redact wallet addresses when a log is intended for public support bundles.
- Include `request_id` on every inbound request and propagate it to background tasks where practical.
- Keep field names stable; add new fields rather than renaming required fields.