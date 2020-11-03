#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;

mod utils;

#[get("/")]
fn index() -> &'static str{
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect{
    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command{
        "tw" => String::from("https://twitter.com"),
        "ddg" => String::from("https://duckduckgo.com"),
        "wiki" => String::from("https://wikipedia.org"),
        _ => utils::google::construct_google_search_url(&cmd)
    };

    Redirect::to(redirect_url)
}



fn main() {
    rocket::ignite().mount("/", routes![index,search]).launch();
}

