# Flyio Rust!

This is an example app demonstrating how to deploy a Rust program on Fly.io

### fly.toml

A fairly standard `fly.toml` configuration, except for the `cmd` override:

```toml
[experimental]
cmd = "./hello" # should be the name of the binary you want to run
```

## Deploy

Once you've went through the steps of `flyctl init`:

```
flyctl deploy
```
