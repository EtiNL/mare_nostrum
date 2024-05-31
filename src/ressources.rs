pub mod ressources_mod  {
    enum RessourceType {
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
        Legendaire(RessourceLegendaires),
    }

    enum RessourceLegendaire {
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