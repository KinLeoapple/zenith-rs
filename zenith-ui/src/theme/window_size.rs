use web_sys::window;

#[derive(PartialEq, Copy, Clone, Debug, Eq)]
pub struct  WindowSize {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
    pub xxl: &'static str,
}

impl WindowSize {
    pub fn new() -> WindowSize {
        WindowSize {
            xs: "xs",
            sm: "sm",
            md: "md",
            lg: "lg",
            xl: "xl",
            xxl: "xxl",
        }
    }

    pub fn get_window_size(&self) -> (i32, i32) {
        let window = window().unwrap();
        let document = window.document().unwrap();

        if document.document_element().is_some() {
            (document.document_element().unwrap().client_width(),
             document.document_element().unwrap().client_height())
        } else {
            (document.body().unwrap().client_width(),
             document.body().unwrap().client_height())
        }
    }

    pub fn breakpoint(&self, width_pixel: i32) -> &str {
        if width_pixel < 576 {
            self.xs
        } else if width_pixel >= 576 && width_pixel < 768 {
            self.sm
        } else if width_pixel >= 768 && width_pixel < 992 {
            self.md
        } else if width_pixel >= 992 && width_pixel < 1200 {
            self.lg
        } else if width_pixel >= 1200 && width_pixel < 1400 {
            self.xl
        } else {
            self.xxl
        }
    }
}