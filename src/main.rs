mod ui;
mod handlers;
mod utils;

use ui::window::Window;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    dotenv::dotenv().ok();

    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "tucha",
        options,
        Box::new(|_cc| Box::<Window>::default()),
    )
}