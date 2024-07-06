slint::include_modules!();


const TAXPER: f64 = .30;
const OWNERPER: f64 = .55;
const PROFITPER: f64 = .05;
const OPEXPER: f64 = .10;
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divid_incoe(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.run()
}

