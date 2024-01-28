![Logo](https://github.com/slavskrit/wspr/blob/b74d575e0531606159dd633f789cd71b208384a8/logo.png?raw=true)

# Dockerized Whisper + Rust + Telegram API bot

To run please execute:

```shell
docker run -e TELOXIDE_TOKEN=YOUR_TG_TOKEN -e RUST_BACKTRACE=1 wspr
```

```shell
docker build -t wspr .
```

```shell
docker save -o wspr.tar wspr:latest
```

## On device

```shell
docker load -i wspr.tar
```

