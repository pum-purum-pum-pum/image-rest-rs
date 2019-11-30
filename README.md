# Image-rest-rs
Rust toy REST server example for uploading images
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
(carefull with docker-compose for while developing it does not rebuild by default. Use ```sudo docker-compose up --build```)
