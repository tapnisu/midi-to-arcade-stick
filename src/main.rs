use std::thread;
use std::time::Duration;
use vigem_client::TargetId;

fn main() {
    let client = vigem_client::Client::connect().unwrap();
    let mut target = vigem_client::Xbox360Wired::new(client, TargetId::XBOX360_WIRED);

    target.plugin().unwrap();

    target.wait_ready().unwrap();

    let gamepad_pressed = vigem_client::XGamepad {
        buttons: vigem_client::XButtons!(A),
        thumb_lx: 256,
        ..Default::default()
    };

    let gamepad_empty = vigem_client::XGamepad::default();

    loop {
        let _ = target.update(&gamepad_pressed);
        thread::sleep(Duration::from_millis(500));

        let _ = target.update(&gamepad_empty);
        thread::sleep(Duration::from_millis(500));
    }
}
