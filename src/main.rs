// Exercice 1 & 2 — Modules et pub use
//
// Grâce au pub use dans lib.rs, on importe directement depuis tp8::
// au lieu de tp8::models::user::User (chemin interne long)
use tp8::{User, Product, Store};
use tp8::utils;

fn main() {
    // Créer un utilisateur (email est privé, on passe par new())
    let user = User::new("Alice", "alice@mail.com");
    println!("Utilisateur : {user}");
    println!("Email : {}", user.email()); // via le getter

    utils::separator();

    // Créer un magasin avec des produits
    let mut store = Store::new();
    store.add_product(Product::new("Clavier", 49.99));
    store.add_product(Product::new("Souris", 29.99));
    store.add_product(Product::new("Écran", 299.99));

    println!("Produits disponibles :");
    store.list_products();

    utils::separator();

    // Achat
    Store::purchase(&user, &store.products[0]);
}
