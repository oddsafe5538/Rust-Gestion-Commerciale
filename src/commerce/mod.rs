use std::{collections::HashMap, iter::Product};

pub mod client;
pub mod product;
use crate::utils;
use client::Client;
use product::Produit;

pub struct GestionCommerciale {
    clients: HashMap<u32, Client>,
    produits: HashMap<u32, Produit>,
}

impl GestionCommerciale {
    pub fn new() -> GestionCommerciale {
        GestionCommerciale {
            clients: HashMap::new(),
            produits: HashMap::new(),
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

    pub fn afficher_tous_les_clients(&self) {
        for (_, client) in self.clients.iter() {
            let Client {
                id,
                name,
                email,
                adress: _,
                solde,
            } = client;
            println!(
                "Liste des clients:\n id: {} nom: {} email: {}  solde: {}",
                id, name, email, solde
            );
        }
    }

    pub fn ajouter_produit(&mut self, nom: String, prix: f32, stock: u32) {
        let id = utils::generate_random_id();
        let product = product::Produit::new(id, nom, prix, stock);
        self.produits.insert(id, product);
    }
}
