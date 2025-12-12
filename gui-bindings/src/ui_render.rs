use std::error::Error;

use slint::ComponentHandle;

slint::include_modules!();

pub fn render_ui() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    println!("testing attr");

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;
    Ok(())
}
