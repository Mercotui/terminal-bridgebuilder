# Terminal Bridgebuilder

A bridgebuilder game for the terminal

## Enabling Logs

To enable the logs, you can use the environment variables, following the tracing [enviroment filter](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html):

```shell
RUST_LOG=TRACE
```

As the terminal is used to display the game, the application logs to STDERR, which we need to redirect to a file:

```shell
RUST_LOG=TRACE terminal-bridgebuilder 2>temp.log
```

Or another terminal:

```shell
tty
# example output /dev/pts/2
```

```shell
RUST_LOG=TRACE terminal-bridgebuilder 2>/dev/pts/2
```
