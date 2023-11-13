use super::{Screen, Style};

impl Screen {
    pub fn set_iterno_max(&mut self, iterno_max: Option<usize>) -> &mut Self {
        self.iterno_max = iterno_max;
        self
    }

    pub fn set_fps_max(&mut self, fps_max: f64) -> Result<&mut Self, &str> {
        if fps_max.is_nan() {
            Err("value cannot be NaN")?
        }
        if fps_max.is_sign_negative() {
            Err("value cannot be negative")?
        }
        self.fps_max = fps_max;
        Ok(self)
    }

    pub fn show_stats(&mut self) -> &mut Self {
        self.show_stats = true;
        self
    }

    pub fn hide_stats(&mut self) -> &mut Self {
        self.show_stats = false;
        self
    }

    pub fn set_style(
        &mut self,
        x_offset: usize,
        y_offset: usize,
        section_sep: usize,
        label_width: usize,
        value_width: usize,
    ) -> &mut Self {
        self.style = Style {
            x_offset,
            y_offset,
            section_sep,
            label_width,
            value_width,
        };
        self
    }
}
