[![WIP](https://img.shields.io/badge/status-WIP-yellow.svg)](https://github.com/kurtismassey/constellation)
[![Checks](https://github.com/kurtismassey/constellation/actions/workflows/checks.yaml/badge.svg)](https://github.com/kurtismassey/constellation/actions/workflows/checks.yaml)


# Constellation

## API Routes

```rust
Router::new()
    .route("/", get(routes::root::get))
    .route("/health", get(routes::health::get))
    .route("/query", post(routes::query::post))
```

#### Request

```bash
curl http://localhost:8080/
```

##### Response

```json
{
  "message": "Welcome to Constellation!"
}
```

### Query endpoint

The query endpoint utilises a web research agent to provide cited responses to queries, the agent is built using [`rig-core` ](https://github.com/0xPlaygrounds/rig/tree/main).

#### Query Request

```bash
curl -X POST -H "Content-Type: application/json" -d '{"query": "what is a constellation?"}' http://localhost:8080/query
```

### Query Response

```json
{
  "message": "Query received",
  "query": "what is a constellation?",
  "response": "A constellation often refers to a group of stars that, when lines are drawn between them, resemble a certain shape that has been named. These stars are far from Earth and are not connected to each other. Some stars in a constellation may be close together, while others are very distant. However, if you were to draw lines in the sky between the stars—like a dot-to-dot puzzle—and use a lot of imagination, the picture would look like a specific object, animal, or person[^1^].\n\nOver time and across different cultures, constellations have been given various names, largely based on what people thought the star patterns resembled. Constellations are influenced by geographical location and time of the year. Nowadays, astronomers still use constellations to name stars and meteor showers[^1^].\n\n[^1^]: https://spaceplace.nasa.gov/constellations/en/"
}
```
