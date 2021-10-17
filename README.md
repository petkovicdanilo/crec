# crec - Container Registry Client

`crec` is a simple CLI application for communicating with OCI registries.

## Features

With `crec` you can pull:
- catalog of all images in registry
- image configuration
- image index (list of all available manifests)
- image manifest
- image layer and save it to disk
- image in OCI format
- list of all avaliable tags for given image

`crec` can communicate with every OCI compliant registry
when given its url.
It supports authentiaction via Bearer token.

[![asciicast](https://asciinema.org/a/442809.svg)](https://asciinema.org/a/442809)
## Installation
To be able to install `crec` you need to
install [Rust](https://www.rust-lang.org/tools/install) programming language tools.

After that, run:
```bash
$ cargo install --git https://github.com/petkovicdanilo/crec
```

