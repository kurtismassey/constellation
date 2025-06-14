# Constellation

## API Routes

```rust
Router::new()
    .route("/", get(routes::root::get))
    .route("/health", get(routes::health::get))
```

#### Request

```bash
curl http://localhost:8080/
```

#### Response

```json
{
  "message": "Welcome to Constellation!"
}
```
