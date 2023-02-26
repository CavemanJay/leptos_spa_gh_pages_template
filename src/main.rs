#![warn(clippy::pedantic, clippy::nursery)]
// mod components;
// mod pages;
use leptos::*;
use leptos_spa_gh_pages_template::*;
// use crate::{components::*, pages::*};
// use yew::prelude::*;
// use yew_router::prelude::*;

#[macro_export]
macro_rules! declare {
    ($mod:ident,$component:ident) => {
        mod $mod;
        pub use $mod::$component;
    };
}


pub fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <RouterExample/> })
}
