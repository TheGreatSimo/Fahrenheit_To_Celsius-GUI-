use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My app",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp{
//    text_fahrenheit_to_celsius : String,
  //  text_celsius_fahrenheit : String,
    text_fahrenheit : String,
    result_of_fahrenheit: f64,
    show_fahrenheit_result_heading : bool,
    text_celsius : String,
    result_of_celsius: f64,
    show_celsius_result_heading : bool,

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.collapsing("Convert Fahrenheit to Celsius", |ui| {
                ui.label("Enter Fahrenheit value");
                ui.text_edit_singleline(& mut self.text_fahrenheit);
                if ui.button("convert").clicked() {
                    let mut text_fahrenheit_input = self.text_fahrenheit.trim().parse().expect("something went wrong !!");
                    self.result_of_fahrenheit = convert_fahrenheit_to_celsius(text_fahrenheit_input);
                    println!("haja {}",self.result_of_fahrenheit);
                    self.show_fahrenheit_result_heading= true
                }
                if self.show_fahrenheit_result_heading {
                    ui.heading(self.result_of_fahrenheit.to_string());
                }
            });
            ui.collapsing("Convert Celsius to Fahrenheit", |ui| {
                ui.label("Enter Celsius value");
                ui.text_edit_singleline(& mut self.text_celsius);
                if ui.button("convert").clicked() {
                    let mut text_celsius_input = self.text_celsius.trim().parse().expect("Weird Stuff happens here");
                    self.result_of_celsius = convert_celsius_fahrenheit(text_celsius_input);
                    self.show_celsius_result_heading = true
                }
                if self.show_celsius_result_heading {
                    ui.heading(self.result_of_celsius.to_string());
                }
            });
        });
    }
}


fn convert_fahrenheit_to_celsius(x :f64) -> f64 {
    (x as f64 - 32.0) * 5.0 / 9.0
}

fn convert_celsius_fahrenheit(x:f64) -> f64 {
    (x as f64 * 9.0 / 5.0) + 32.0
}
