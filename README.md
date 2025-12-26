# Blob-service
Written in Rust, for educational purposes
## Install and launch
Clone the repo
```
git clone https://github.com/distraw/blob-service
```
Build the project
```
cd blob-service
cargo build
```
Build the docker image and start the compose
```
docker build -t blob-service .
docker compose up
```
Server is ready to run!

## Run the client
You have two options – to create note or fetch an existing one

Create new note:
```
cargo run -p client create-note "your note text"
```

Fetch the existing one:
```
cargo run -p client get-note <id>
```
_* ID MUST be an integer (i32), for example `cargo run -p client get-note 1`_