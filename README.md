# rsevm
quick and easy server side rendering in rust

## tasks

- [ ] route parameters
  ```rust
  server.get("/users/:id", Box::new(|response| {
    let id = response.get_param("id").unwrap();
  }))
  ```