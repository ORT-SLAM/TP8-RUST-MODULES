// On importe les types depuis nos modules models
// crate:: = la racine du projet (comme / dans un chemin de fichiers)
use crate::models::product::Product;
use crate::models::user::User;

pub struct Store {
    pub products: Vec<Product>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn list_products(&self) {
        for product in &self.products {
            println!("  - {product}");
        }
    }

    // Fonction utilitaire qui combine User et Product
    pub fn purchase(user: &User, product: &Product) {
        println!("{} a acheté {}", user.name, product);
    }
}
