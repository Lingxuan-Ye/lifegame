use super::Matrix;
use std::fmt::Display;

impl<T: Clone + Default + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_len: usize = 0;
        let reprs: Vec<String> = self
            .data
            .iter()
            .map(|value| {
                let repr = format!("{value:.2}");
                if repr.len() > max_len {
                    max_len = repr.len();
                }
                repr
            })
            .collect();

        let mut rows: Vec<String> = Vec::with_capacity(self.shape.0);
        let mut items: Vec<String> = Vec::with_capacity(self.shape.1);
        for row in reprs.chunks(self.shape.1) {
            for item in row {
                items.push(format!("{item:>max_len$}"))
            }
            rows.push(format!("    [{}],", items.join(", ")));
            items.clear()
        }
        write!(f, "[\n{}\n]", rows.join("\n"))
    }
}
