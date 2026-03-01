// Exercice 2 — pub use pour exposer une API propre
//
// mod = "ce module existe" (déclare l'existence)
// pub use = "re-exporte cet élément à la racine de la crate"
//
// Sans pub use, un utilisateur devrait écrire :
//   tp8::models::user::User
//   tp8::models::product::Product
//   tp8::services::store::Store
//
// Avec pub use, il peut écrire simplement :
//   tp8::User
//   tp8::Product
//   tp8::Store

// On déclare les modules (organisation interne)
pub mod models;
pub mod services;
pub mod utils;

// On re-exporte les types importants à la racine (API publique)
pub use models::user::User;
pub use models::product::Product;
pub use services::store::Store;
