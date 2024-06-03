use crate::ressources::ressources_mod::RessourceType;
use crate::heroes_and_wonders::heroes_and_wonders_mod::{HeroesAndWondersEnum, Ability};
use crate::plateau::plateau::TerritoireEnum;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    heroes_and_wonders: Vec<HeroesAndWondersEnum>,
    ressources: Vec<(RessourceType, u8)>,
    territoires: Vec<TerritoireEnum>,
    abilities: Vec<Ability>,
}

impl Player {
    fn new_game() -> Vec<Player> {
        let mut players = Vec::new();

        let mut roman_player = Player {
                                                name: "Empire Romain".to_string(),
                                                heroes_and_wonders: vec![HeroesAndWondersEnum::Cesar],
                                                ressources: vec![(RessourceType::Cereales,1), (RessourceType::Mouton,1),
                                                                (RessourceType::Vin,1), (RessourceType::Metal,2),
                                                                (RessourceType::Huile,1)],
                                                territoires: vec![TerritoireEnum::Latium, TerritoireEnum::Gallia, TerritoireEnum::Italia],
                                                abilities: HeroesAndWondersEnum::Cesar.get_ability(),
                                            };

        let mut carthaginois_player = Player {
                                                    name: "Empire Carthaginois".to_string(),
                                                    heroes_and_wonders: vec![HeroesAndWondersEnum::Hannibal],
                                                    ressources: vec![(RessourceType::Impot,1), (RessourceType::Or,1),
                                                                    (RessourceType::Gladiateur,1), (RessourceType::Mouton,2),
                                                                    (RessourceType::Parfum,1), (RessourceType::Huile,1)],
                                                    territoires: vec![TerritoireEnum::Numidia, TerritoireEnum::Africa, TerritoireEnum::Libya],
                                                    abilities: HeroesAndWondersEnum::Hannibal.get_ability(),
                                                    };

        let mut greek_player = Player {
                                                name: "Empire Grec".to_string(),
                                                heroes_and_wonders: vec![HeroesAndWondersEnum::Pericles],
                                                ressources: vec![(RessourceType::Huile,1), (RessourceType::Impot,2),
                                                                (RessourceType::Cereales,1), (RessourceType::Vin,1),
                                                                (RessourceType::Marbre,1)],
                                                territoires: vec![TerritoireEnum::Thracia, TerritoireEnum::Macedonia, TerritoireEnum::Achea],
                                                abilities: HeroesAndWondersEnum::Pericles.get_ability(),
                                            };

        let mut egypsian_player = Player {
                                                    name: "Empire Egyptien".to_string(),
                                                    heroes_and_wonders: vec![HeroesAndWondersEnum::Cleopatre],
                                                    ressources: vec![(RessourceType::Impot,5), (RessourceType::Papyrus,2),
                                                                    (RessourceType::Cereales,1), (RessourceType::Or,1)],
                                                    territoires: vec![TerritoireEnum::Cyrenaica, TerritoireEnum::Aegyptus, TerritoireEnum::Aethiopia],
                                                    abilities: HeroesAndWondersEnum::Cleopatre.get_ability(),
                                                };

        players.push(roman_player);
        players.push(carthaginois_player);
        players.push(greek_player);
        players.push(egypsian_player);
        
        players
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PlayerEnum {
    Roman,
    Carthaginois,
    Greek,
    Egypsian
}

impl PlayerEnum {

    fn get_player_hashmap(players: Vec<Player>) ->  HashMap<PlayerEnum, Player> {
        let mut player_map: HashMap<PlayerEnum, Player> = HashMap::new();
    
        for player in players {
            let player_enum = match player.name.as_str() {
                "Empire Romain" => PlayerEnum::Roman,
                "Empire Carthaginois" => PlayerEnum::Carthaginois,
                "Empire Grec" => PlayerEnum::Greek,
                "Empire Egyptien" => PlayerEnum::Egypsian,
                _ => continue,
            };
            player_map.insert(player_enum, player);
        }

        player_map
    }
}