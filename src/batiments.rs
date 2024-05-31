use ressources::RessourceType;
pub mod Batiments {
    pub enum BatimentsTypes {
        Caravane(RessourceType),
        Marche(Vect<&Caravane>),
        Cite(RessourceType),
        Capitale(RessourceType),
        CiteLegendaire(RessourceType),
        Temple(Batiments),
        Forteresse,
    }
}