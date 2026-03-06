use chrono::{DateTime, Local};
use gtk::glib::{self, Date, clone, timeout_add_seconds_local};
use gtk::{pango, prelude::*};
use gtk::{Builder, Application, ApplicationWindow, Button, Label};


fn set_time(date_time_label: &Label) -> () {
    let now: DateTime<Local> = Local::now();
    let time_str = now.format("%B %-d, %Y %I:%M:%S %p").to_string();
    date_time_label.set_label(time_str.as_str());
}

fn main() {
    let red_color: &str = "#e0e01b1b2424";
    let green_color: &str = "#2e2ec2c27e7e";

    let app = Application::builder()
        .application_id("com.example.UIFile")
        .build();

    app.connect_activate(|app| {
        let builder = Builder::from_file("./ui/time_tracker_main.glade");

        let window: ApplicationWindow = builder.object("main_window")
            .expect("Failed to get main_window");

        let error_label: Label = builder.object("error_label")
            .expect("Failed to get error_label");

        let start_stop_button: Button = builder.object("start_stop_button")
            .expect("Failed to get start_stop_button");

        let date_time_label: Label = builder.object("date_time_label")
            .expect("Failed to get date_time_label");

        // Set initial time
        set_time(&date_time_label);

        // Global Timer
        timeout_add_seconds_local(1, move || {
            set_time(&date_time_label);
            glib::Continue(true)
        });

        start_stop_button.connect_clicked(move |button| {
            if button.label().unwrap() == "Start" {
                button.set_label("Stop");
                error_label.set_text("Timer running...");
            } else {
                button.set_label("Start");
                error_label.set_text("");
            }
        });
        
        window.set_application(Some(app));
        window.show();
    });

    app.run();
}