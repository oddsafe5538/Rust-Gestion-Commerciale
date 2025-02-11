pub struct Commande {
    id: u32,
    client_id: u32,
    produits: Vec<(u32, u32)>,
}

impl Commande {
    pub fn new(id: u32, client_id: u32, produits: Vec<(u32, u32)>) -> Commande {
        Commande {
            id,
            client_id,
            produits,
        }
    }
}
