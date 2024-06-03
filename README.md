# ip-locator
ip-locator is a web app meant to be a simple rust/htmx clone of whatismyip.com

## Usage
The app will be hosted on port 8000, and can be accessed from a web browser. 

There is a minimal docker image of this app available on docker hub, listed as [removingnest109/iplocator:latest](https://hub.docker.com/repository/docker/removingnest109/iplocator)

To run on docker, port 8000 from the container will need to be mapped to a port on the host

## Installation (Cargo)
#### Clone the repo
```bash
git clone https://github.com/removingnest109/ip-locator.git

cd ip-locator
```
Now you will need to either run the app, build the app, or install the app 

#### To run the app once in debug mode
```bash
cargo run
```

#### To build the app to binary for manual installation
```bash
cargo build --release
```

#### To install into ~/.cargo/bin/
```bash
cargo install --path .
```

## Installation (Docker)
The docker image is built using static linked musl binaries on a minimal scratch image.

The result is a 20MB image that compresses to 6MB on docker hub.

### To install:

#### Pull the image from docker hub
```bash
docker pull removingnest109/iplocator:latest
```
#### Run the image in a new container
```bash
docker run -p 8000:8000 removingnest109/iplocator:latest
```

## Todo
 - [ ] Add map view for geolocation
 
