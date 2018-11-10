// #![windows_subsystem = "windows"]

#[macro_use] extern crate nickel;
extern crate web_view;

use nickel::{Nickel, HttpRouter};
use web_view::{run, Content};
use std::{thread};

fn main() {
    // start web server thread
    thread::spawn(move || {
        let mut server = Nickel::new();
        server.get("/api/hello", middleware!("Hello World"));
        server.get("/main.js", middleware!(include_str!("./static/main.js")));
        server.get("/styles.css", middleware!(include_str!("./static/styles.css")));
        server.get("**", middleware!(include_str!("./static/index.html")));
        server.listen("127.0.0.1:3000").expect("couldn't start server");
    });

    // start web view in gui/main thread
	let size = (800, 600);
	let resizable = true;
	let debug = true;
	let init_cb = |_webview| {};
	let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
	let userdata = ();
    run(
        "Noman",
        Content::Url("http://127.0.0.1:3000"),
        Some(size),
        resizable,
        debug,
        init_cb,
        frontend_cb,
        userdata
    );
}
