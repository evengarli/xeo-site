use std::env;
use xeo::Xeo; 

fn main() {
   
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port_str.parse().expect("PORT must be a number");
    let mut app = Xeo::new(port, "0.0.0.0".to_string());
    app.route("/".to_string(), "index.html".to_string());
    app.route("/docs".to_string(), "docs.html".to_string());
    app.server("".to_string());
}
