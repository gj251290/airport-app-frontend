#![allow(non_snake_case)]

mod app;
mod components;
mod constants;
mod models;
mod pages;
mod state;

fn main() {
    dioxus::launch(app::App);
}
