pub(crate) struct Settings {
    pub(crate) sensitivity: f32,
}

impl Settings {
    pub(crate) fn new(sens: f32) -> Self {
        Self { sensitivity: sens }
    }
}
