## Description

A CLI tool to check and fix relations managed by 'Ory Keto' for the hub-permissions service.

## Installation

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone the repository and navigate to the directory.
3. Run `cargo build --release`

## Configuration

Ensure you have the correct database credentials in your config file. use [config.sample.json](config.sample.json) as a guide.
You can set the config path using the `CONFIG_PATH` environment variable or `--config` argument on each command.

## Usage

```bash
# Check user relation (member)
hub-permissions-cli check user <id> --relation member

# Check user relation (owners) and fix if necessary
hub-permissions-cli check user <id> --relation owners --fix

# Check and fix all mints
hub-permissions-cli check mints --all --fix

# Check a single drop
hub-permissions-cli check drop <id>

# Check and fix all credentials
hub-permissions-cli check credentials --all --fix
```

> Replace <id> with a valid UUID v4.

## Using from docker

A Docker image is available at `holaplex/hub-permissions-cli:latest`.

To use it, run:

```bash
docker run -it --rm -v $(pwd)/config.json:/app/config.json holaplex/hub-permissions-cli:latest check user 123e4567-e89b-12d3-a456-426614174000 --relation member
```

## Contributing

If you find any issues or would like to contribute to the project, feel free to open an issue or create a pull request on the repository.
