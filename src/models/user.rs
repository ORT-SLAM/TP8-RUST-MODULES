// La struct est pub, mais email est privé → on force le passage par new()
pub struct User {
    pub name: String,
    email: String, // privé : on ne veut pas qu'on modifie l'email directement
}

impl User {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    // Getter pour lire l'email sans pouvoir le modifier
    pub fn email(&self) -> &str {
        &self.email
    }
}

// Display pour pouvoir faire println!("{}", user)
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}
