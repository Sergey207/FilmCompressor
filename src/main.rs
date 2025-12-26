mod app;

use std::io;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let app_result = app::App::new().run(terminal);
    ratatui::restore();
    app_result
}
