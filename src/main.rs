use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Builder, Entry, Button, ListBox, ListBoxRow};

const GLADE_UI: &str = include_str!("window.glade");

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.example.SnappyToDo")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let builder = Builder::from_string(GLADE_UI);

    let window: ApplicationWindow = builder
        .object("main_window")
        .expect("Could not find main_window");

    window.set_application(Some(app));

    let task_entry: Entry = builder
        .object("task_entry")
        .expect("Could not find task_entry");

    let add_button: Button = builder
        .object("add_button")
        .expect("Could not find add_button");

    let task_list: ListBox = builder
        .object("task_list")
        .expect("Could not find task_list");

    add_button.connect_clicked(move |_| {
        // Sadly, it's only practical to remake the whole builder object to duplicate an item like this
        let temporary_builder = Builder::from_string(GLADE_UI);

        let task_placeholder: ListBoxRow = temporary_builder
            .object("task_placeholder")
            .expect("Could not find task_placeholder");

        let task_text: Entry = temporary_builder
            .object("task_text")
            .expect("Could not find task_text");

        let delete_button: Button = temporary_builder
            .object("delete_button")
            .expect("Could not find delete_button");

        let entered_text: String = task_entry.property("text");
        task_entry.set_property("text", "");
        task_text.set_property("text", &entered_text);

        task_list.add(&task_placeholder);

        delete_button.connect_clicked(move |_| {
            unsafe {
                task_placeholder.destroy();
            }
        });
    });

    window.present();
}