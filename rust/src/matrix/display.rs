use super::Matrix;
use std::fmt::Display;

impl<T: Clone + Default + Display> Matrix<T> {
    fn fmt_with_width(&self, width: usize) -> String {
        let mut row_reprs: Vec<String> = Vec::with_capacity(self.shape.0 + 2);
        let mut elem_reprs: Vec<String> = Vec::with_capacity(self.shape.1 + 2);
        for row in self.data.chunks(self.shape.1) {
            for col in row {
                elem_reprs.push(format!("{col:>width$.3}"));
            }
            row_reprs.push(format!("    [{}],", elem_reprs.join(", ")));
            elem_reprs.clear();
        }
        format!("[\n{}\n]", row_reprs.join("\n"))
    }
}

impl Display for Matrix<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(u8::MAX.to_string().len()))
    }
}

impl Display for Matrix<u16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(u16::MAX.to_string().len()))
    }
}

impl Display for Matrix<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(u32::MAX.to_string().len()))
    }
}

impl Display for Matrix<u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(u64::MAX.to_string().len()))
    }
}

impl Display for Matrix<u128> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(u128::MAX.to_string().len()))
    }
}

impl Display for Matrix<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(usize::MAX.to_string().len()))
    }
}

impl Display for Matrix<i8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(i8::MIN.to_string().len()))
    }
}

impl Display for Matrix<i16> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(i16::MIN.to_string().len()))
    }
}

impl Display for Matrix<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(i32::MIN.to_string().len()))
    }
}

impl Display for Matrix<i64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(i64::MIN.to_string().len()))
    }
}

impl Display for Matrix<i128> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(i128::MIN.to_string().len()))
    }
}

impl Display for Matrix<isize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(isize::MIN.to_string().len()))
    }
}

impl Display for Matrix<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(8))
    }
}

impl Display for Matrix<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(8))
    }
}

impl Display for Matrix<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(5))
    }
}

impl Display for Matrix<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_width(1))
    }
}
