# rsevm
quick and easy server side rendering in rust

## tasks

- [x] route parameters
  ```rust
  server.get("/users/:id", Box::new(|response| {
    let id = response.get_param("id").unwrap();
  }))
  ```

- [x] post requests & body data
  ```rust
  server.post("/new-user", Box::new(|req, res| {
  	let body = req.get_body();

  	answer_text(res, 200, body["message"])
  }))
  ```
