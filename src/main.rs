#[macro_use] extern crate rocket;

use std::process::{Command, ExitStatus};
use rocket::{response::content::Html};

#[get("/")]
fn index() -> Html<&'static str> {
    
    
    unsafe {
        let content = include_bytes!("index.html");
        return Html(std::str::from_utf8_unchecked(content));
    }
    
}

fn exit_code_to_user_message(exit_status: ExitStatus) -> String {
    if exit_status.success() { 
        return "Shutting down...".to_string();
    } else {
        return format!("Shutdown error: {}", exit_status);
    }
    //format!("shutdown exit code {}", exit_code)
}

#[get("/shutdown")]
fn shutdown() -> String {
    
    //let status = Command::new("shutdown.exe /s /t 1").status();
    let status = Command::new("shutdown.exe")
        .args(["/s", "/t", "5"])
        .status();

    match status {
        Ok(exit_code) => exit_code_to_user_message(exit_code),
        Err(error) => format!("Internal server error: {}", error),
    }
}

#[get("/cancel")]
fn cancel() -> String {
    
    //let status = Command::new("shutdown.exe /s /t 1").status();
    let status = Command::new("shutdown.exe")
        .args(["/a"])
        .status();

    match status {
        Ok(exit_code) => format!("shutdown (cancel) exit code {}", exit_code),
        Err(error) => format!("Internal server error: {}", error),
    }
}




async fn rocket_main() {
    // Recall that an uninspected `Error` will cause a pretty-printed panic,
    // so rest assured failures do not go undetected when using `#[launch]`.
    let _ = rocket::build()
        .mount("/", routes![index, shutdown, cancel])
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
