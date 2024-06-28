use std::process::exit;

mod modules;

#[tokio::main]
async fn main() {

    let _ = modules::tui::tui();

    exit(0);

}
