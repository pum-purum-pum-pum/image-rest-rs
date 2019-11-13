# Image-rest-rs
Rust REST server for uploading images
# Run
## Native
```cargo run --release --p <PORT> --o <OUTPUT_DIRECTORY>``` (see cli.yml for details)
## With docker
Create docker image:
```docker image build -t image_rest .```
Run docker image:
```docker run -p 8000:8000 image_rest:latest```
## With docker compose
```docker-compose up```