#[derive(PartialEq)]
pub enum Size {
    Sm,
    Md,
    Lg,
}

impl Size {
    pub fn get(&self) -> (i32, i32) {
        match self {
            Size::Sm => {todo!()}
            Size::Md => (100, 45),
            Size::Lg => {todo!()}
        }
    }
}

