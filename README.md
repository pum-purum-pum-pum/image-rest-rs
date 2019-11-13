# Image-rest-rs
Rust REST server for uploading images
# Run
## Native
```cargo run --release --p <PORT> --o <OUTPUT_DIRECTORY>``` (see cli.yml for details)
## With docker
Create docker image:
```sudo docker image build -t image_rest .```
Run docker image:
```sudo docker run image_rest -p 8000:8000```
## With docker compose
```sudo docker-compose up```