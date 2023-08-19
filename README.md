# WebApp with Axum

Welcome to the heart of backend efficiency and security! Our Rust project, powered by the potent Axum framework from Tokio, seamlessly integrates with the robustness of PostgreSQL hosted within a Docker container featuring persistent storage. Leveraging the enchantment of Sea-ORM, our data models spring to life, supporting an encrypted JWT authentication system and robust session control. Following the Model-View-Controller (MVC) architecture, our endpoints facilitate the management of to-do tasks through CRUD operations, all guarded by custom middleware layers. The future beckons with an expansion of concurrency through Tokio's capabilities, upholding the gold standard of memory safety that only Rust can provide. Join us in unleashing the might of Rust in the backend! Happy coding.

### Backend technologies :
-  Axum
-  Sea-ORM 
-  Postgresql
-  Docker 

### Build

- Start Docker container with Postgresql DB:

  ```
  docker-compose up -d --wait
  ```

- Init Server:

```
  cargo watch -x run
```