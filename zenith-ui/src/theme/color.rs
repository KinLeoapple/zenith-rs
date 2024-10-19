#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    _50,
    _100,
    _200,
    _300,
    _400,
    _500,
    _600,
    _700,
    _800,
    _900,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Common {
    White,
    Black,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Background {
    Body,
    Surface,
    Popup,
    Level1,
    Level2,
    Level3,
    Tooltip,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Text {
    Primary,
    Secondary,
    Tertiary,
    Icon,
}

impl Theme {
    pub fn theme(&self) -> &str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark"
        }
    }
}

impl Color {
    pub fn primary(&self) -> &str {
        match self {
            Color::_50 => "#EDF5F5",
            Color::_100 => "#E3EFFB",
            Color::_200 => "#C7DFF7",
            Color::_300 => "#97C3F0",
            Color::_400 => "#4393E4",
            Color::_500 => "#0B6BCB",
            Color::_600 => "#185EA5",
            Color::_700 => "#12467B",
            Color::_800 => "#0A2744",
            Color::_900 => "#051423"
        }
    }

    pub fn neutral(&self) -> &str {
        match self {
            Color::_50 => "#FBFCFE",
            Color::_100 => "#F0F4F8",
            Color::_200 => "#DDE7EE",
            Color::_300 => "#CDD7E1",
            Color::_400 => "#9FA6AD",
            Color::_500 => "#636B74",
            Color::_600 => "#555E68",
            Color::_700 => "#32383E",
            Color::_800 => "#171A1C",
            Color::_900 => "#0B0D0E"
        }
    }

    pub fn danger(&self) -> &str {
        match self {
            Color::_50 => "#0B0D0E",
            Color::_100 => "#FCE4E4",
            Color::_200 => "#F7C5C5",
            Color::_300 => "#F09898",
            Color::_400 => "#E47474",
            Color::_500 => "#C41C1C",
            Color::_600 => "#A51818",
            Color::_700 => "#7D1212",
            Color::_800 => "#430A0A",
            Color::_900 => "#240505"
        }
    }

    pub fn success(&self) -> &str {
        match self {
            Color::_50 => "#F6FEF6",
            Color::_100 => "#E3FBE3",
            Color::_200 => "#C7F7C7",
            Color::_300 => "#C7F7C7",
            Color::_400 => "#51BC51",
            Color::_500 => "#1F7A1F",
            Color::_600 => "#136C13",
            Color::_700 => "#0A470A",
            Color::_800 => "#042F04",
            Color::_900 => "#021D02"
        }
    }

    pub fn warning(&self) -> &str {
        match self {
            Color::_50 => "#FEFAF6",
            Color::_100 => "#FDF0E1",
            Color::_200 => "#FCE1C2",
            Color::_300 => "#F3C896",
            Color::_400 => "#EA9A3E",
            Color::_500 => "#9A5B13",
            Color::_600 => "#72430D",
            Color::_700 => "#492B08",
            Color::_800 => "#2E1B05",
            Color::_900 => "#1D1002"
        }
    }
}

impl Common {
    pub fn common(&self) -> &str {
        match self {
            Common::White => "#FFF",
            Common::Black => "#000"
        }
    }
}

impl Background {
    pub fn dark(&self) -> &str {
        match self {
            Background::Body => Common::Black.common(),
            Background::Surface => Color::_900.neutral(),
            Background::Popup => Common::Black.common(),
            Background::Level1 => Color::_800.neutral(),
            Background::Level2 => Color::_700.neutral(),
            Background::Level3 => Color::_600.neutral(),
            Background::Tooltip => Color::_600.neutral(),
        }
    }

    pub fn light(&self) -> &str {
        match self {
            Background::Body => Common::White.common(),
            Background::Surface => Color::_50.neutral(),
            Background::Popup => Common::White.common(),
            Background::Level1 => Color::_100.neutral(),
            Background::Level2 => Color::_200.neutral(),
            Background::Level3 => Color::_300.neutral(),
            Background::Tooltip => Color::_500.neutral(),
        }
    }
}

impl Text {
    pub fn dark(&self) -> &str {
        match self {
            Text::Primary => Color::_100.neutral(),
            Text::Secondary => Color::_300.neutral(),
            Text::Tertiary => Color::_400.neutral(),
            Text::Icon => Color::_400.neutral(),
        }
    }

    pub fn light(&self) -> &str {
        match self {
            Text::Primary => Color::_800.neutral(),
            Text::Secondary => Color::_700.neutral(),
            Text::Tertiary => Color::_600.neutral(),
            Text::Icon => Color::_500.neutral(),
        }
    }
}