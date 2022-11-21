use std::{process::{Command, Stdio}, os::unix::process::CommandExt, str::{self, FromStr}};
use fltk::{*, macros::window, window::Window, prelude::{WidgetExt, GroupExt, MenuExt}, frame::Frame};

fn other() -> String {
    let ifconf = Command::new("ifconfig")
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
    let awk = Command::new("awk")
    .arg("-F")
    .arg(" ")
    .arg("{print $1}")
    .stdin(Stdio::from(ifconf.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

    let raw_out = awk.wait_with_output().unwrap();
    let read_out = str::from_utf8(&raw_out.stdout).unwrap();
    let returnable = read_out.to_owned();

    return returnable;
}

fn into(x: String, y: String) {
    println!("{}{}", x, y );
}

fn main() {
    let app = app::App::default();

    let listing = &other().clone();
    let list = listing.split("\n");
    let mut vlist: Vec<&str> = list.collect();


    let mut wind = Window::default()
    .with_size(500, 500)
    .with_label("Testing");
    
    let mut choices = menu::Choice::default()
    .with_label("Test Choices: ")
    .with_size(200, 20)
    .center_x(&wind);

    for i in &vlist {
        choices.add_choice(i);
    }

    let mut but = button::Button::default()
    .with_label("Button")
    .with_size(200, 30)
    .below_of(&choices, 10);

    let mut frame = Frame::default()
    .with_size(200, 200)
    .below_of(&but, 10);

    but.set_callback(move |_| {
        into(choices.choice().unwrap(), String::from("Hello"));
        frame.set_label(&choices.choice().unwrap());
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
