use crate::ressources::ressources_mod::RessourceType;
use crate::heroes_and_wonders::heroes_and_wonders_mod::{HeroesAndWondersEnum, Ability};
use crate::plateau::plateau::Territoire;

#[derive(Debug, Clone)]
pub struct Player {
    heroes_and_wonders: Vec<HeroesAndWondersEnum>,
    ressources: Vec<(RessourceType, u8)>,
    territoires: Vec<&'static Territoire>,
    abilities: Vec<Ability>,
}