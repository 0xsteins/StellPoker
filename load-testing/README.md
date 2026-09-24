# Load Testing Environment

This folder contains a local `k6` load testing environment with Grafana dashboards and Prometheus scraping for the coordinator service.

## What is included

- `docker-compose.yml` starts:
  - InfluxDB for k6 result storage
  - Prometheus for scraping coordinator metrics from `/metrics`
  - Grafana for real-time dashboards
  - k6 container for running benchmark scripts
- `k6/benchmark.js` simulates concurrent table/player activity with configurable
  session status, action submission, proof submission, and state queries
- Grafana dashboards are provisioned automatically from `grafana/dashboards`

## Start the environment

From the repository root:

```bash
cd load-testing
docker compose up -d
```

Then open:

- Grafana: `http://localhost:3000` (`admin` / `admin`)
- Prometheus: `http://localhost:9090`
- InfluxDB: `http://localhost:8086`

## Run a load test

Use the `k6` container to execute the benchmark against your locally-running coordinator service.

Example: 100 concurrent tables

```bash
cd load-testing
docker compose run --rm -e BASE_URL=http://host.docker.internal:8080 -e TABLE_COUNT=100 -e VUS=100 -e DURATION=2m k6 run --out influxdb=http://influxdb:8086/k6 /scripts/benchmark.js
```

Example: 500 concurrent tables

```bash
cd load-testing
docker compose run --rm -e BASE_URL=http://host.docker.internal:8080 -e TABLE_COUNT=500 -e VUS=500 -e DURATION=3m k6 run --out influxdb=http://influxdb:8086/k6 /scripts/benchmark.js
```

Example: 1000 concurrent tables

```bash
cd load-testing
docker compose run --rm -e BASE_URL=http://host.docker.internal:8080 -e TABLE_COUNT=1000 -e VUS=1000 -e DURATION=4m k6 run --out influxdb=http://influxdb:8086/k6 /scripts/benchmark.js
```

To exercise a real session and write endpoints, provide a session identifier,
the table-relative action/proof paths, and JSON request bodies. The paths and
bodies are deliberately environment variables because valid payloads depend
on the deployed table and player accounts:

```bash
docker compose run --rm \
  -e BASE_URL=http://host.docker.internal:8080 \
  -e SESSION_ID="$SESSION_ID" \
  -e ACTION_PATH="/api/table/$TABLE_ID/player-action" \
  -e ACTION_BODY='{"player":"...","action":"check"}' \
  -e PROOF_PATH="/api/table/$TABLE_ID/request-showdown" \
  -e PROOF_BODY='{}' \
  -e VUS=100 -e DURATION=3m \
  k6 run --out influxdb=http://influxdb:8086/k6 /scripts/benchmark.js
```

## Finding the throughput limit

Run the same workload at increasing `VUS` values (for example 100, 250, 500,
and 1000), keeping the table count and duration fixed. Record requests per
second, p95/p99 latency, error rate, coordinator CPU/memory, and MPC session
queue depth from Grafana. Treat the sustainable limit as the highest level
that remains below the 5% error threshold and the p99 latency threshold for
the full run; report the first level that violates either threshold as the
observed saturation point. These are environment-specific measurements, not
hard protocol limits, so include CPU, memory, database, MPC-node count, and
commit SHA with every result.

## Metrics reported

- P99 latency from k6
- Error rate from k6
- Throughput from k6
- Coordinator CPU usage
- Coordinator memory usage

## Notes

- The coordinator must be running locally and reachable on `http://localhost:8080`
- `host.docker.internal` is used for container-to-host networking on Docker Desktop
- If Grafana does not automatically load the dashboard, refresh the dashboards list in Grafana
