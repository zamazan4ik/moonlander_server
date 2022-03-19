#[derive(Debug)]
pub struct LayoutChanged {
    pub current_layout: u32,
}

impl dbus::arg::ReadAll for LayoutChanged {
    fn read(i: &mut dbus::arg::Iter) -> Result<Self, dbus::arg::TypeMismatchError> {
        Ok(LayoutChanged {
            current_layout: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for LayoutChanged {
    const NAME: &'static str = "layoutChanged";
    const INTERFACE: &'static str = "org.kde.KeyboardLayouts";
}

fn main() {
    let conn = dbus::blocking::Connection::new_session().expect("Cannot connect to dbus");
    let proxy = conn.with_proxy(
        "org.kde.keyboard",
        "/Layouts",
        std::time::Duration::from_millis(5000),
    );

    {
        let _id = proxy.match_signal(
            |h: LayoutChanged, _: &dbus::blocking::Connection, _: &dbus::Message| {
                println!("Layout changed. Current layout: {}", h.current_layout);

                let buf = [0u8, h.current_layout as u8];
                let api = hidapi::HidApi::new().unwrap();
                let path = std::ffi::CString::new("1-8:1.1").unwrap();
                let device = api.open_path(path.as_c_str()).unwrap();
                device.write(&buf).unwrap();
                true
            },
        );
    }

    loop {
        conn.process(std::time::Duration::from_millis(1000));
    }
}
