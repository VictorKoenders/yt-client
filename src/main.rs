mod api;
mod yt;

#[async_std::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let mut app = tide::with_state(State::default());
    app.at("/").serve_file("static/index.html").unwrap();
    app.at("/static").serve_dir("static").unwrap();

    api::mount(app.at("/api"));

    app.listen("0.0.0.0:8081").await.unwrap();
}

#[derive(Default, Clone)]
pub struct State {}
