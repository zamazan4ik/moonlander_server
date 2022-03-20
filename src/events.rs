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
