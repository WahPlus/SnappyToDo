use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, Entry, ListBox, ListBoxRow, glib};

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

        let up_button: Button = temporary_builder
            .object("up_button")
            .expect("Could not find up_button");

        let down_button: Button = temporary_builder
            .object("down_button")
            .expect("Could not find down_button");

        let entered_text: String = task_entry.property("text");
        task_entry.set_property("text", "");
        task_text.set_property("text", &entered_text);

        task_list.add(&task_placeholder);

        let task_clone = task_placeholder.clone();
        let task_list_clone = task_list.clone();
        up_button.connect_clicked(move |_| {
            let current_position = task_clone.index();
            if current_position != 0 {
                task_list_clone.remove(&task_clone);
                task_list_clone.insert(&task_clone, current_position - 1);
            }
        });

        let task_clone = task_placeholder.clone();
        let task_list_clone = task_list.clone();
        down_button.connect_clicked(move |_| {
            let current_position = task_clone.index();
            // The entry just won't move, no need for an if
            task_list_clone.remove(&task_clone);
            task_list_clone.insert(&task_clone, current_position + 1);
        });

        delete_button.connect_clicked(move |_| unsafe {
            task_placeholder.destroy();
        });
    });

    window.present();
}
