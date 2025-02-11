use std::{collections::HashMap, iter::Product};

pub mod client;
pub mod commande;
pub mod product;
use crate::utils;
use client::Client;
use commande::Commande;
use product::Produit;

pub struct GestionCommerciale {
    clients: HashMap<u32, Client>,
    produits: HashMap<u32, Produit>,
    commandes: HashMap<u32, Commande>,
}

impl GestionCommerciale {
    pub fn new() -> GestionCommerciale {
        GestionCommerciale {
            clients: HashMap::new(),
            produits: HashMap::new(),
            commandes: HashMap::new(),
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

    pub fn creer_commande(&mut self, client_id: u32, produits: Vec<(u32, u32)>) -> () {
        let id = utils::generate_random_id();
        self.commandes
            .insert(id, Commande::new(id, client_id, produits));
    }

    fn calculer_total(&self, produits: Vec<(u32, u32)>) -> f32 {
        let mut total: f32 = 0.0;

        for (produit_id, quantite) in produits.iter() {
            match self.produits.get(produit_id) {
                Some(produit) => total += (*quantite as f32) * produit.get_price(),
                None => {}
            }
        }

        total
    }
}
