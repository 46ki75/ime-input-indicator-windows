extern crate systray;

fn main() {
    if let Err(e) = try_main() {
        println!("Error: {}", e);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = systray::Application::new()?;
    let _ = app.set_icon_from_file("assets/icon.ico")?;
    let _ = app.add_menu_item("Say Hello", |_| {
        println!("Hello, world!");
        Ok::<_, systray::Error>(())
    })?;
    let _ = app.add_menu_item("Exit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    app.wait_for_message()?;
    Ok(())
}
