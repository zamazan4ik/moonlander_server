mod events;
mod logging;
mod parameters;

fn main() {
    tracing::info!("Moonlander server started");
    logging::init_logger();

    let parameters =
        envy::from_env::<parameters::Parameters>().expect("Cannot load config from env");

    let dbus_connection = dbus::blocking::Connection::new_session().expect("Cannot connect to dbus");
    let dbus_keyboard_proxy = dbus_connection.with_proxy(
        "org.kde.keyboard",
        "/Layouts",
        std::time::Duration::from_millis(5000),
    );

    {
        let _id = dbus_keyboard_proxy.match_signal(
            move |h: events::LayoutChanged, _: &dbus::blocking::Connection, _: &dbus::Message| {
                tracing::debug!("Layout changed to {}", h.current_layout);
                let notify_result = notify_keyboard(h.current_layout as u8, parameters.device_path.clone());

                if let Err(error) = notify_result {
                    tracing::error!("Error during keyboard notification: {}", error);
                }

                true
            },
        );
    }

    loop {
        let signal_processing_result = dbus_connection.process(std::time::Duration::from_millis(1000));

        if let Err(error) = signal_processing_result {
            tracing::error!("Error during Dbus signal processing: {}", error);
        }
    }
}

fn notify_keyboard(current_layout: u8, device_path: String) -> Result<(), Box<dyn std::error::Error>> {
    // First byte in the message is skipped is used for message type identification - just put zero
    let message_with_layout = [0u8, current_layout];
    let api = hidapi::HidApi::new()?;
    let path = std::ffi::CString::new(device_path)?;
    let device = api.open_path(path.as_c_str())?;
    device.write(&message_with_layout)?;

    Ok(())
}
