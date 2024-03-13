# How to run the server
- Install Rust: https://www.rust-lang.org/tools/install
- Go to the `server` folder
- Run `cargo r` (or `cargo r -r` for faster runtime)
- Connect to http://localhost:3000/login.html
- Enjoy!

# If the port `3000` is being used
- Go to the `server/src/main.rs` file
- In the line 19 (which is `let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000))).await?;`), change the number `3000` to other port you want

Now you can connect to http://localhost:{your port}/login.html as usual