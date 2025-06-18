# Dioxus Todo App

A full-stack Todo application built with Rust. The frontend is a Single Page Application (SPA) built with the [Dioxus](https://dioxuslabs.com/) framework, and the backend is a REST API built with [Axum](https://github.com/tokio-rs/axum).

## Features

*   Create, read, update, and delete todos.
*   Full-stack Rust application.
*   Containerized with Docker for easy setup and deployment.

## Tech Stack

### Frontend

*   [Dioxus](https://dioxuslabs.com/): A Rust framework for building cross-platform user interfaces.
*   [dioxus-router](https://dioxuslabs.com/router/): For client-side routing.
*   [reqwest](https://crates.io/crates/reqwest): For making HTTP requests to the backend.
*   [gloo-storage](https://crates.io/crates/gloo-storage): For interacting with browser's local storage.
*   [Nginx](https://www.nginx.com/): To serve the static frontend assets.

### Backend

*   [Axum](https://github.com/tokio-rs/axum): A web application framework that focuses on ergonomics and modularity.
*   [SQLx](https://github.com/launchbadge/sqlx): A modern, async, and type-safe SQL client for Rust.
*   [SQLite](https://www.sqlite.org/): As the database.
*   [Tokio](https://tokio.rs/): An asynchronous runtime for Rust.

## Getting Started

### Prerequisites

*   [Docker](https://www.docker.com/get-started)
*   [Docker Compose](https://docs.docker.com/compose/install/)

### Running the application

1.  Clone the repository:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2.  Build and run the application using Docker Compose:
    ```sh
    docker-compose up --build
    ```

The frontend will be available at `http://localhost:8080` and the server will be running on `http://localhost:3000`.

## Project Structure

```
.
├── docker-compose.yml      # Docker compose for running services
├── Dockerfile.frontend     # Dockerfile for the frontend
├── Dockerfile.server       # Dockerfile for the backend
├── dioxus-todo-app/        # Dioxus frontend application
│   ├── src/
│   ├── Cargo.toml
│   └── Dioxus.toml
├── server/                 # Axum backend application
│   ├── src/
│   ├── migrations/
│   └── Cargo.toml
└── Cargo.toml              # Rust workspace configuration
``` 