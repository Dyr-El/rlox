#[derive(Clone, Copy)]
pub struct Value(f64);

impl From<f64> for Value {
    fn from(f: f64) -> Value {
        Value(f)
    }
}

impl Value {
    pub fn print(&self) {
        print!("{}", self.0)
    }
}
