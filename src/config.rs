pub(crate) struct Settings {
    pub(crate) enabled: bool,
    pub(crate) sensitivity: f32,
}

impl Settings {
    pub(crate) fn new(enabled: bool, sens: f32) -> Self {
        Self {
            enabled,
            sensitivity: sens,
        }
    }
}
