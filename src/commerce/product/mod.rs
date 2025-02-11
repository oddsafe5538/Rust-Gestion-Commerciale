pub struct Produit {
    id: u32,
    nom: String,
    prix: f32,
    stock: u32,
}

impl Produit {
    pub fn new(id: u32, nom: String, prix: f32, stock: u32) -> Produit {
        Produit {
            id,
            nom,
            prix,
            stock,
        }
    }

    pub fn get_price(&self)-> f32 {
        self.prix
    }
}
