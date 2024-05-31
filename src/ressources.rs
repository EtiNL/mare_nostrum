pub mod ressources_mod  {
    pub enum RessourceType {
        Ceramique,
        Diamant,
        Papyrus,
        Metal,
        Parfum,
        Marbre,
        Bois,
        Or,
        Cereales,
        Huile,
        Mouton,
        Vin,
        Gladiateur,
        Impot,
        Legendaire(RessourceLegendaire),
    }

    pub enum RessourceLegendaire {
        Ceramique,
        Diamant,
        Papyrus,
        Metal,
        Parfum,
        Marbre,
        Bois,
        Or,
        Cereales,
        Huile,
        Mouton,
        Vin,
        Gladiateur,
    }
}