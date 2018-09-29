extern crate client;
extern crate yew;

mod b_component;
mod router;
mod routing;

use client::Model;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
