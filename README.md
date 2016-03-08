GTK-REST examples
=================

This repository contains servers & clients to establish a plain text POST communication between them. Its main purpose was to research on various programming languages that may be of interest for both servers and GTK+ applications development.

These packages were made and are intended to run on (GNU)Linux distributions, but some of them seem to work on Windows too.

Everything is licensed as public domain, so do whatever you want with it.

## Usage

First clone the repository.

    git clone https://github.com/espectalll123/gtk-rest-examples.git

In order to execute an example, first check if you have their required dependencies:

- GTK+ 3.16 or newer
- node.js
- Gjs
  - LibSoup
- Julia 0.4
  - Merly.jl
  - Gtk.jl
  - Requests.jl
- Python 3.x (may also work with 2.x)
  - CherryPy
  - PyGObject
  - Requests
- Rust 1.5 or newer
- Valac 0.30 or newer
  - LibSoup

After that, `cd` to the folder with the programming language that you're intending to test. Then, setup and run the code:

### Javascript
#### Server

    npm install
    sudo npm start

#### Client

    gjs ./client.js

### Julia
#### Server

    sudo julia ./server.jl

#### Client

    julia ./client.js

### Python
#### Server
    
    sudo python ./server.js

#### Client

    python ./client.js

### Rust
#### Server

    cd server
    cargo build
    sudo cargo run

#### Client

    cd client
    cargo build
    cargo run

If you want to run one of the servers without root permissions, just set the port in the source code.
