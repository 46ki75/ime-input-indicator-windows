// Create a Windows application that resides in the system tray.
// When the user clicks on a text box and the IME text cursor appears,
// display a tooltip indicating the current input mode, whether it's
// half-width (Hankaku) or full-width (Zenkaku) input.

extern crate systray;
extern crate winapi;

// Specific imports from the winapi crate to get functions and types needed for our implementation.
use winapi::shared::minwindef::HKL;
use winapi::um::winuser::{GetForegroundWindow, GetKeyboardLayout};

// Explicitly link to imm32 library. This is necessary to call functions from this DLL.
#[link(name = "imm32")]
extern "system" {
    // Import the ImmGetOpenStatus function, which returns the open status of an Input Method Editor (IME).
    fn ImmGetOpenStatus(himc: HKL) -> i32;
}

// Function to check if IME is enabled for the current application window.
fn is_ime_enabled() -> bool {
    unsafe {
        // Get the handle to the foreground window.
        let _hwnd = GetForegroundWindow();

        // Get the handle to the keyboard layout.
        let hkl = GetKeyboardLayout(0);

        // Check the open status of the IME. If ImmGetOpenStatus returns non-zero, IME is enabled.
        ImmGetOpenStatus(hkl) != 0
    }
}

// Entry point of the application.
fn main() {
    // If the main functionality encounters an error, print it out.
    if let Err(e) = try_main() {
        println!("Error: {}", e);
    }
}

// Main functionality of the application.
fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new system tray application.
    let mut app = systray::Application::new()?;

    // Set an icon for the system tray application.
    let _ = app.set_icon_from_file("assets/icon.ico")?;

    // Add menu items to the system tray application.
    let _ = app.add_menu_item("Say Hello", |_| {
        println!("Hello, world!");
        Ok::<_, systray::Error>(())
    })?;
    let _ = app.add_menu_item("Exit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    // Spawn a new thread that will periodically check if IME is enabled and update the tooltip accordingly.
    std::thread::spawn(|| loop {
        if is_ime_enabled() {
            // クロージャ内でapp.set_tooltipを呼ぶ代わりに、ここで処理
        } else {
            // クロージャ内でapp.set_tooltipを呼ぶ代わりに、ここで処理
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    });

    // Wait for messages/events on the main thread.
    app.wait_for_message()?;

    Ok(())
}
