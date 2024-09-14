use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Fahrenheit/Celsius Conversion",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    text_fahrenheit: String,
    result_of_fahrenheit: Option<f64>,
    show_fahrenheit_result_heading: bool,
    text_celsius: String,
    result_of_celsius: Option<f64>,
    show_celsius_result_heading: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.collapsing("Convert Fahrenheit to Celsius", |ui| {
                ui.label("Enter Fahrenheit value");
                ui.text_edit_singleline(&mut self.text_fahrenheit);
                if ui.button("Convert").clicked() {
                    if let Ok(fahrenheit) = self.text_fahrenheit.trim().parse::<f64>() {
                        self.result_of_fahrenheit = Some(convert_fahrenheit_to_celsius(fahrenheit));
                        self.show_fahrenheit_result_heading = true;
                    } else {
                        self.result_of_fahrenheit = None;
                        self.show_fahrenheit_result_heading = false;
                    }
                }
                if self.show_fahrenheit_result_heading {
                    if let Some(result) = self.result_of_fahrenheit {
                        ui.heading(format!("{:.10} °C", result));
                    }
                }
            });

            ui.collapsing("Convert Celsius to Fahrenheit", |ui| {
                ui.label("Enter Celsius value");
                ui.text_edit_singleline(&mut self.text_celsius);
                if ui.button("Convert").clicked() {
                    if let Ok(celsius) = self.text_celsius.trim().parse::<f64>() {
                        self.result_of_celsius = Some(convert_celsius_to_fahrenheit(celsius));
                        self.show_celsius_result_heading = true;
                    } else {
                        self.result_of_celsius = None;
                        self.show_celsius_result_heading = false;
                    }
                }
                if self.show_celsius_result_heading {
                    if let Some(result) = self.result_of_celsius {
                        ui.heading(format!("{:.10} °F", result));
                    }
                }
            });
        });
    }
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
