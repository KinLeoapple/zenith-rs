#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Size {
    Sm,
    Md,
    Lg,
}

impl Size {
    pub fn get(&self) -> (i32, i32) {
        match self {
            Size::Sm => (40, 24),
            Size::Md => (80, 48),
            Size::Lg => (120, 72)
        }
    }
}

