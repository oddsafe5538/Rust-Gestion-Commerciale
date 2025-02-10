use std::collections::HashMap;

pub mod client;
use crate::utils;
use client::Client;

pub struct GestionCommerciale {
    clients: HashMap<u32, Client>,
}

impl GestionCommerciale {
    pub fn new() -> GestionCommerciale {
        GestionCommerciale {
            clients: HashMap::new(),
        }
    }
    pub fn ajouter_client(
        &mut self,
        name: String,
        email: String,
        adress: String,
        solde: f64,
    ) -> () {
        let id: u32 = utils::generate_random_id();
        let new_client = client::Client::nouveau_client(id, name, email, adress, 0.0);
        self.clients.insert(id, new_client);
        self.ajouter_solde(id, solde);
    }

    pub fn ajouter_solde(&mut self, id: u32, solde: f64) {
        match self.clients.get_mut(&id) {
            Some(client) => client.set_sold(solde),
            None => println!("Aucun client ne correspond à cet ID"),
        }
    }
}
