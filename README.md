# Minimal Rust Web Server with Rocket

## Overview

This project demonstrates a minimal, simple web server built in Rust using the Rocket framework. Rocket provides a powerful yet easy-to-use foundation for web applications in Rust, focusing on speed, safety, and flexibility.

## Building Image Using Procfile Instead Of Dockerfile

Instead of using a traditional `Dockerfile`, this project leverages a `Procfile` for buildpack-based deployments. The `Procfile` specifies the command needed to run the compiled Rust application, making it straightforward to deploy on platforms that support buildpacks. For more information, please refer to the `Procfile` file.
```procfile
web: /workspace/bin/hello_rocket
```

## Building with Buildpacks

The build process utilizes buildpacks, which automatically detect the presence of a `Procfile` to configure the application's run command. This method simplifies the deployment process, as it abstracts away the need for a Dockerfile and manual image configuration. For detailed instructions, please refer to the `woksflow/rust-buildpack.yaml` file.
```yaml
name: Rust - Buildpack

on:
  push:
    branches: [ "master" ]
  pull_request:
    branches: [ "master" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Log in to Docker Hub and Push the image
      uses: docker/login-action@v2
      with:
      username: ${{ secrets.DOCKER_USERNAME }}
      password: ${{ secrets.DOCKER_PASSWORD }}

    - name: Install Pack CLI
      uses: buildpacks/github-actions/setup-pack@v5.0.0

    - name: Build the image using the Rust buildpack
      run: |
      pack build ${{ secrets.DOCKER_USERNAME }}/rust-rocket-buildpacks --buildpack docker.io/paketocommunity/rust --builder paketobuildpacks/builder-jammy-full --path .
    
    - name: Push the image to Docker Hub
      run: |
      docker push ${{ secrets.DOCKER_USERNAME }}/rust-rocket-buildpacks:latest
```

## Pushing to Docker Registry

After building the application with buildpacks, the resulting image can be pushed to a Docker registry. This step involves tagging the built image appropriately and using `docker push` to upload it to the registry, making it available for deployment.

This approach combines the ease of buildpacks with the flexibility and portability of Docker, offering a streamlined path from development to deployment.