mod commerce;
mod utils;

use commerce::GestionCommerciale;

fn main() {
    let mut gestion = GestionCommerciale::new();
    gestion.ajouter_client(
        "Alice".to_string(),
        "alice@example.com".to_string(),
        "123 Rue Principale".to_string(),
        100.0,
    );

    gestion.afficher_tous_les_clients();
    gestion.ajouter_produit("Livre".to_string(), 20.0, 5);
}
