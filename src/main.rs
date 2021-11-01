#[macro_use] extern crate rocket;

use std::process::Command;
use rocket::response::content::Html;

#[get("/")]
fn index() -> Html<&'static str> {
    
    
    unsafe {
        let content = include_bytes!("index.html");
        return Html(std::str::from_utf8_unchecked(content));
    }
    
}

#[get("/shutdown")]
fn shutdown() -> String {
    
    //let status = Command::new("shutdown.exe /s /t 1").status();
    let status = Command::new("shutdown.exe")
        .args(["/s", "/t", "2"])
        .status();

    match status {
        Ok(exit_code) => format!("shutdown exit code {}", exit_code),
        Err(error) => format!("Internal server error: {}", error),
    }
}




async fn rocket_main() {
    // Recall that an uninspected `Error` will cause a pretty-printed panic,
    // so rest assured failures do not go undetected when using `#[launch]`.
    let _ = rocket::build()
        .mount("/", routes![index, shutdown])
        .launch()
        .await;
}

// async fn keyboard_interrupt_handler() {
    
//     let stdin = io::stdin();
//     let mut line = String::new();
//     let _ = stdin.read_line(&mut line).await;

//     let shutdown = rocket::Shutdown::notify();


//     rocket::Shutdown::notify()
    
// }

#[rocket::main]
async fn main() {
    //let _ = futures::join!(rocket_main(), keyboard_interrupt_handler());
    rocket_main().await;
}
