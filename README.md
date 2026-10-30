# Sagemcom Fast Prometheus Exporter

A lightweight Prometheus exporter for Sagemcom F@st series routers.

## Features

- Scrapes metrics from Sagemcom F@st routers via HTTP API
- Prometheus-compatible metrics endpoint
- Supports SHA512 and MD5 authentication methods
- Configurable logging levels

## Building

```console
cargo build --release
```

### Cross-compile for Raspberry Pi (aarch64)

```console
rustup target add aarch64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-musl
```

## Usage

```console
sagemcom-fast-exporter --host 192.168.2.1 --password ABC123456789
```

### Command Line Options

- `--host <address>` - Router IP address (required)
- `--port <port>` - Router API port (default: 80)
- `--username <username>` - Username (default: admin)
- `--password <password>` - Password (required)
- `--bind <address>` - Listen address (default: 127.0.0.1:9780)
- `--refresh-interval <seconds>` - Token refresh interval (default: 5m)
- `--auth-method <method>` - Authentication method: SHA512 or MD5 (default: SHA512)
- `--log-level <level>` - Log level: trace, debug, info, warn, error (default: info)

### Endpoints

- `/scrape` - Returns router metrics in Prometheus format
- `/metrics` - Returns exporter metrics (scrape duration, success)
- `/health` - Health check endpoint

## Grafana Dashboard

The Grafana dashboard at [Grafana.com](https://grafana.com/grafana/dashboards/20374) by @hairyhenderson should work with this project.

## Credits

This Rust implementation is a port of the [Go exporter](https://github.com/hairyhenderson/sagemcom_fast_exporter) by @hairyhenderson.

The API protocol was reverse-engineered from the [python-sagemcom-api](https://github.com/iMicknl/python-sagemcom-api) project by @iMicknl.

## License

[MIT License](../LICENSE)
