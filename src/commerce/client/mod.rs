pub struct Client {
    id: u32,
    name: String,
    email: String,
    adress: String,
    solde: f64,
}

impl Client {
    pub fn nouveau_client(
        id: u32,
        name: String,
        email: String,
        adress: String,
        solde: f64,
    ) -> Client {
        Client {
            id,
            name,
            email,
            adress,
            solde,
        }
    }

    pub fn set_sold(&mut self, solde: f64) {
        self.solde += solde;
    }
}
