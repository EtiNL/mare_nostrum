use crate::ressources::ressources_mod::{RessourceType, RessourceLegendaire};

pub mod batiments {
    use super::{RessourceType, RessourceLegendaire};

    #[derive(Debug, Clone)]
    pub enum BatimentsTypes {
        Caravane(RessourceType),
        Marche,
        Cite,
        Capitale,
        CiteLegendaire,
        Temple,
        Forteresse,
    }
}
