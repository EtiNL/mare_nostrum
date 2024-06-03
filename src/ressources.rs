
pub mod ressources_mod  {

    #[derive(Debug, Clone)]
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

    #[derive(Debug, Clone)]
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