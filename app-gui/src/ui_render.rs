use std::error::Error;

use app_core::equations;
use slint::ComponentHandle;

slint::include_modules!();

pub fn render_ui() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_calculate({
        let ui_handle = ui.as_weak();
        move |input_a: slint::SharedString,
              input_b: slint::SharedString,
              op: slint::SharedString| {
            let ui = ui_handle.unwrap();

            let a = input_a.trim().parse::<f64>();
            let b = input_b.trim().parse::<f64>();

            match (a, b) {
                (Ok(a), Ok(b)) => {
                    let result = match op.as_str() {
                        "+" => equations::add(a, b),
                        "−" | "-" => equations::subtract(a, b),
                        _ => {
                            ui.set_result("Unknown op".into());
                            return;
                        }
                    };
                    ui.set_result(format!("{result}").into());
                }
                _ => {
                    ui.set_result("Invalid input".into());
                }
            }
        }
    });

    ui.run()?;
    Ok(())
}
