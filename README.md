 ## Rust Client with Auth0 
 
 
 This client makes an HTTP requests an access token from Auth0 to access an API. 
 
 
 ## Overview
 
In this Rust project, I define two structs: AuthRequestBody and AuthResponseBody, which correspond to the request and response bodies of the OAuth 2.0 token request, respectively. I'm using the serde crate to serialize the request body into JSON, and to deserialize the response body from JSON.

I'm then creating a reqwest client, setting the appropriate URL and headers for the OAuth 2.0 token endpoint, and sending the AuthRequestBody as JSON in a POST request. If the response is successful, I'm printing the access token from the AuthResponseBody. Otherwise, I'm printing the status code of the response.

## Step by Step Guide

1. Create a Virtual Environment

The purpose of virtual environments is to create a self-contained environment for each of your projects, allowing you to manage dependencies, libraries, and versions separately for each project.

    `python3 -m venv rustenv`

    `source rustenv/bin/activate`

    `cd rustenv`

2. Install Rust
Go to https://rustup.rs/ and run the command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 

    Run `source "$HOME/.cargo/env"` to configure your current shell.

3. create new project
The cargo tool is the default package manager for Rust and provides an easy way to manage dependencies and build projects.

    Run `cargo new` (project name) (my Eg: `cargo new hello`)

This will create a binary (application) `microservice` package

4. Create `main.rs` and `lib.rs` files in the src project

    `touch main.rs` and `touch lib.rs` 

5. Run `Cargo build`   
This is a command in the Rust programming language that is used to compile a Rust project. It compiles the project's source code and its dependencies, and produces an executable binary file. The cargo build command can be run from the root directory of the project.
