# Thread Backend Server written Rust
- Using Axum 0.8, Sqlx 0.8

## Project Strcutre
```
- src/ 
    - api/ # Controller layer, defines API endpoints
    - config/ # App state, routes, environment variables
    - domain/ # DTO, DB Model
    - middleware/ # Middleware (e.g. JWT token verification)
    - repository/ # Repository layer (async trait -> impl)
    - service/ # Service layer (contains business logic)
    - utils/ # Utility functions and helpers
    - error.rs # Custom error types
    - main.rs # Entry point, starts the server
- migrations/ # SQL files for database schema migrations
- test/ # Test Code (using reqwest crates)
```