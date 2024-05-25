# PuVS

This repository is all about diving into the basics of parallel and distributed systems using Docker. It features the creation of a todo application as a practical exercise in understanding these concepts.

## Frameworks

The application is implemented using the following frameworks

### Rocket.rs [🕳](https://rocket.rs)

* "**Rocket** is an async web framework for Rust with a focus on usability, security, extensibility, and speed." [©](https://github.com/rwf2/Rocket/tree/v0.5)

### ~~Yew.rs~~ [🕳](https://yew.rs) (not required to be implemented)

* "**Yew** is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly." [©](https://github.com/yewstack/yew)

## Usage

### Prerequisites

* [Docker](https://docs.docker.com/engine/install/) installed

### Installation

1. Clone this repository to your local machine:

```bash
git clone https://github.com/Disgame/PuVS.git
```

2. Navigate to the project directory:

```bash
cd PuVS
```

3. Run docker compose

```bash
docker compose -f "docker-compose-rust.yml" up (--build to rebuild it locally)
```

## Docker Compose Configuration

This Docker Compose configuration defines a network and two services:

- `todo-db`: This service sets up a MongoDB database for storing todos. It builds an image using the `todo-db.dockerfile` located in the `./mongoDB-todo-db` directory and exposes port `27017` for MongoDB connections.
- `todo-api`: This service hosts the Rocket-based Todo API. It builds an image using the `todo-api.dockerfile` located in the `./rocket-todo-api` directory and exposes port `8000` for API connections. It depends on the `todo-db` service to be running, as it requires the MongoDB database to be available.

The services are connected to the same network (`todo-network`), allowing them to communicate with each other.

## API Endpoints (http://localhost:8000)

### `GET /`

Returns a welcome message with information about the available endpoints.

```

Hello, this is my Todos Application, see them at /todos!

Create a new one with POST /todos/<name>

Delete one with DELETE /todos/<name>

```

### `GET /todos`

Returns a list of todos.

Example Response:

```json

["Todo 1",  "Todo 2",  "Todo 3"]

```

### `POST /todos/<name>`

Creates a new todo with the specified name.

Example Request:

```bash

POST /todos/ExampleTodo

```

Example Response:

```arduino
"ExampleTodo"
```

### `DELETE /todos/<name>`

Deletes a todo with the specified name.

Example Request:

```bash

DELETE /todos/ExampleTodo

```

Example Response:

```arduino
"ExampleTodo"
```

### Error Handling

The API provides a default error handler to redirect to the root path (/) in case of errors.

## Docker Build & Using Docker Hub

First, we need to build the images we want to publish.

Todo Database:

```bash
cd mongoDB-todo-db

docker build -t disgame/lab:todo-api -f "todo-api.dockerfile" .
```

Todo Backend:

```bash
cd rocket-todo-api

docker build -t disgame/lab:todo-api -f "todo-api.dockerfile" .
```


### Docker Hub

Next, we'll login and push our images to our Docker Hub repository.

```bash
docker login

docker push disgame/lab:todo-api

docker push disgame/lab:todo-db
```
