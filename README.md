# crudcrate-example
An example API implementing **[crudcrate](https://crates.io/crates/crudcrate)**.

## Usage
1. Clone the repository.

```bash
git clone https://github.com/evanjt/crudcrate-example.git
```

2. Run `cargo run` in the project directory.

```bash
cd crudcrate-example
cargo run
```

3. Visit `http://localhost:8000` in your browser.

The docs (Scalar) are available at `http://localhost:8000/docs`. You can
test the API from the docs page by using the test request buttons.

The CRUD endpoints for `todo` are prefixed at `http://localhost:8000/todo`.

### Examples
#### Create three todos
```bash
curl http://0.0.0.0:3000/todo \
  --request POST \
  --header 'Content-Type: application/json' \
  --data '{
    "completed": false,
    "title": "Todo 1",
    "user_readable_title": "First Todo"
}'

curl http://0.0.0.0:3000/todo \
  --request POST \
  --header 'Content-Type: application/json' \
  --data '{
    "completed": false,
    "title": "Todo 2",
    "user_readable_title": "Second Todo"
}'

curl http://0.0.0.0:3000/todo \
  --request POST \
  --header 'Content-Type: application/json' \
  --data '{
    "completed": false,
    "title": "Todo 3",
    "user_readable_title": "Third Todo"
}'
```

#### Get all todos
```bash
curl http://0.0.0.0:3000/todo
```

#### Get one todo (replace {id} with the actual todo ID)
```bash
curl http://0.0.0.0:3000/todo/{id}
```

#### Get todos with a filter (example: filtering todos that contain "Todo" in the title)
```bash
curl "http://0.0.0.0:3000/todo?filter=%7B%22q%22%3A%20%22Todo%22%7D"
```

#### Update a todo (replace {id} with the actual todo ID)
```bash
curl http://0.0.0.0:3000/todo/{id} \
  --request PUT \
  --header 'Content-Type: application/json' \
  --data '{
    "completed": true,
    "title": "Updated Todo",
    "user_readable_title": "Updated Todo"
}'
```

#### Delete a todo (replace {id} with the actual todo ID)
```bash
curl http://0.0.0.0:3000/todo/{id} \
  --request DELETE
```