use crate::ressources::ressources_mod::RessourceType;

pub mod batiments {
    use super::RessourceType;

    #[derive(Debug, Clone)]
    pub enum BatimentsTypes {
        Caravane(RessourceType),
        Marche(Vec<BatimentsTypes>),
        Cite(RessourceType),
        Capitale(RessourceType),
        CiteLegendaire(RessourceType),
        Temple(Vec<BatimentsTypes>),
        Forteresse,
    }

}
