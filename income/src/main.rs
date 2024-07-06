slint::include_modules!();


const TAXPER: f64 = .30;
const OWNERPER: f64 = .55;
const PROFITPER: f64 = .05;
const OPEXPER: f64 = .10;
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax: f64 = num *TAXPER;
        let owner: f64 = num *OWNERPER;
        let profit: f64 = num *PROFITPER;
        let opex: f64 = num *OPEXPER;
        let result=fortmat!("Taxes:{:2}\nOwner:{:2}\nProfit:{:2}\nOpEx:{:2}", {tax}, {owner}, {profit}, {opex})
        ui.set_results(result);
    });

    ui.run()
}

