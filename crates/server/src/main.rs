use miniserve::{Content, Request, Response};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(_req: Request) -> Response {
    Ok(Content::Json("{\"messages\": [\"Hello world!\", \"And how does that make you feel?\"]}".to_string()))
}

fn main() {
    let mut server = miniserve::Server::new();
    server = server.route("/", index);
    server = server.route("/chat", chat);
    server.run();
}
