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
## ideas

- state management: instead of having to use `Arc::new(Mutex::new(...));` for global state variables in the routes. Create a `State` class and a `Reducer` function.
  ```rust
  struct State {
    books: vec<Book>
  }
  
  let mut server = rsevm::Server::new::<State>();
  
  server.get("/books/:id", Box::new(|req, state| {
    match req.get_param(":id") {
      Some(id) => {
        let id = id.parse::<usize>().unwrap();

        Ok((state.books[id].clone(), state))
      },
      None => Err((403, "no id provided".to_string()))
    }
  }));
  
  server.post("/books/:id", Box::new(|req, state| {
    state.books.add(Book::new());
    
    Ok(("created", state))
  }));
  ```
