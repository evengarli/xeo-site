use xeo::Xeo;

fn main() {
    let mut app = Xeo::new(80,"127.0.0.1".to_string());

    app.route("/".to_string(), "index.html".to_string());

    app.server("".to_string());

}
