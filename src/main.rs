use fltk::{app, input::Input, prelude::*, window::Window};
use rdev::{listen, EventType, Key as keyer, Event};
use std::process::{exit, Command};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 150, 100, "Enter Command");
    let input = Input::new(0, 0, 150, 100, "");

    wind.end();
    wind.show();

    std::thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            callback(event, &input);
        }) {
            eprintln!("Error: {:?}", error);
        }
    });

    app.run().unwrap();
}

fn callback(event: Event, input: &Input) {
    match event.event_type {
        EventType::KeyPress(keyer::SemiColon) => {
            let text = input.value();
            process_input(&text);
            exit(0);
        }
        _ => {}
    }
}

fn process_input(input: &str) {
    match input {
        "hello;" => println!("You typed 'hello'!"),
        "exit;" => println!("Exiting..."),
        "quack;" => quack(true),
        "quack 0;" => quack(false),
        _ => println!("Unrecognized command: {}", input),
    }
}

fn quack(arg: bool) {
    if arg {
        Command::new("quack").arg("&").spawn().expect("quack fail");
    } else {
        Command::new("pkill").arg("quack").spawn().expect("shotgun miss");
    }
}
