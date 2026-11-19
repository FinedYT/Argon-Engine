use argon_engine::core::app::App;

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.run().unwrap();
}
