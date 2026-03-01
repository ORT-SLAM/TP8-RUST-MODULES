pub struct Product {
    pub name: String,
    pub price: f64,
}

impl Product {
    pub fn new(name: &str, price: f64) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:.2}€)", self.name, self.price)
    }
}
