# Launch the client command line tool

## First define an environment variable

- VIESSMANN_TOKEN token from the Viessmann website in the 'Generate Access Token' section

```
user@machine: export VIESSMANN_TOKEN=SUPER_TOKEN_123
```

## How to launch

### Using cargo

```
cargo run -- info
```

### Using build binary

```
./target/debug/viessmann-sdk info
```