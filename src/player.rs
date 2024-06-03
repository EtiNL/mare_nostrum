use crate::monopole_counter::monopole_counter::Monopole;
use crate::ressources::ressources_mod::RessourceType;
use crate::heroes_and_wonders::heroes_and_wonders_mod::{HeroesAndWondersEnum, Ability};
use crate::plateau::plateau::TerritoireEnum;
use crate::batiments::batiments::BatimentsTypes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub heroes_and_wonders: Vec<HeroesAndWondersEnum>,
    pub ressources: Vec<(RessourceType, u8)>,
    pub territoires: Vec<TerritoireEnum>,
    pub abilities: Vec<Ability>,
    pub monopoles: (Monopole, Monopole, Monopole),
}

impl Player {
    pub fn ressource_production(&mut self) {
        let mut double_good = false;
        let mut double_tax = false;

        for territoire_enum in TerritoireEnum::iterator(){
            
            let producteur 

            // First pass to identify the effects
            for (batiment, actif) in &producteurs {
                if !actif {
                    continue;
                }
                match batiment {
                    BatimentsTypes::Marche => double_good = true,
                    BatimentsTypes::Temple => double_tax = true,
                    _ => continue,
                }
            }

            let mut ressources = Vec::new();

            // Second pass to generate resources
            for (batiment, actif) in producteurs {
                if !actif {
                    continue;
                }
                let generated = match batiment {
                    BatimentsTypes::Caravane(ressource) => Some((ressource, if double_good { 2 } else { 1 })),
                    BatimentsTypes::Cite => Some((RessourceType::Impot, if double_tax { 2 } else { 1 })),
                    BatimentsTypes::Capitale => Some((RessourceType::Impot, if double_tax { 2 } else { 1 })),
                    BatimentsTypes::CiteLegendaire => Some((RessourceType::Legendaire(RessourceLegendaire::Marbre), 1)),
                    _ => None,
                };
                if let Some(resource) = generated {
                    ressources.push(resource);
                }
            }

            ressources}
        
        
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
    pub fn get_player(hashmap: &HashMap<PlayerEnum, Arc<Mutex<Player>>>, player_enum: &PlayerEnum) -> Option<Arc<Mutex<Player>>> {
        hashmap.get(player_enum).cloned()
    }

    pub fn from_player_name(name: &str) -> Self {
        match name {
            "Empire Romain" => PlayerEnum::Roman,
            "Empire Carthaginois" => PlayerEnum::Carthaginois,
            "Empire Grec" => PlayerEnum::Greek,
            "Empire Egyptien" => PlayerEnum::Egypsian,
            _ => panic!("Unknown player name"),
        }
    }

    pub fn iterator() -> impl Iterator<Item = PlayerEnum> {
        [
            PlayerEnum::Roman,
            PlayerEnum::Carthaginois,
            PlayerEnum::Greek,
            PlayerEnum::Egypsian
        ]
        .iter()
        .cloned()
    }
}


pub fn new_players() -> (Vec<Arc<Mutex<Player>>>, HashMap<PlayerEnum, Arc<Mutex<Player>>>) {
    let mut players = Vec::new();

    let mut roman_player = Arc::new(Mutex::new(Player {
                                            name: "Empire Romain".to_string(),
                                            heroes_and_wonders: vec![HeroesAndWondersEnum::Cesar],
                                            ressources: vec![(RessourceType::Cereales,1), (RessourceType::Mouton,1),
                                                            (RessourceType::Vin,1), (RessourceType::Metal,2),
                                                            (RessourceType::Huile,1)],
                                            territoires: vec![TerritoireEnum::Latium, TerritoireEnum::Gallia, TerritoireEnum::Italia],
                                            abilities: HeroesAndWondersEnum::Cesar.get_ability(),
                                            monopoles: (Monopole::Commercial(7), Monopole::Politique(1), Monopole::Militaire(3))
                                        }));

    let mut carthaginois_player = Arc::new(Mutex::new(Player {
                                                name: "Empire Carthaginois".to_string(),
                                                heroes_and_wonders: vec![HeroesAndWondersEnum::Hannibal],
                                                ressources: vec![(RessourceType::Impot,1), (RessourceType::Or,1),
                                                                (RessourceType::Gladiateur,1), (RessourceType::Mouton,2),
                                                                (RessourceType::Parfum,1), (RessourceType::Huile,1)],
                                                territoires: vec![TerritoireEnum::Numidia, TerritoireEnum::Africa, TerritoireEnum::Libya],
                                                abilities: HeroesAndWondersEnum::Hannibal.get_ability(),
                                                monopoles: (Monopole::Commercial(7), Monopole::Politique(1), Monopole::Militaire(2))
                                                }));

    let mut greek_player = Arc::new(Mutex::new(Player {
                                            name: "Empire Grec".to_string(),
                                            heroes_and_wonders: vec![HeroesAndWondersEnum::Pericles],
                                            ressources: vec![(RessourceType::Huile,1), (RessourceType::Impot,2),
                                                            (RessourceType::Cereales,1), (RessourceType::Vin,1),
                                                            (RessourceType::Marbre,1)],
                                            territoires: vec![TerritoireEnum::Thracia, TerritoireEnum::Macedonia, TerritoireEnum::Achea],
                                            abilities: HeroesAndWondersEnum::Pericles.get_ability(),
                                            monopoles: (Monopole::Commercial(4), Monopole::Politique(4), Monopole::Militaire(3))
                                        }));

    let mut egypsian_player = Arc::new(Mutex::new(Player {
                                                name: "Empire Egyptien".to_string(),
                                                heroes_and_wonders: vec![HeroesAndWondersEnum::Cleopatre],
                                                ressources: vec![(RessourceType::Impot,5), (RessourceType::Papyrus,2),
                                                                (RessourceType::Cereales,1), (RessourceType::Or,1)],
                                                territoires: vec![TerritoireEnum::Cyrenaica, TerritoireEnum::Aegyptus, TerritoireEnum::Aethiopia],
                                                abilities: HeroesAndWondersEnum::Cleopatre.get_ability(),
                                                monopoles: (Monopole::Commercial(4), Monopole::Politique(4), Monopole::Militaire(2))
                                            }));

    players.push(roman_player.clone());
    players.push(carthaginois_player.clone());
    players.push(greek_player.clone());
    players.push(egypsian_player.clone());

    let mut players_map: HashMap<PlayerEnum, Arc<Mutex<Player>>> = HashMap::new();

    for player in &players {
        let player_enum = match player.lock().unwrap().name.as_str() {
            "Empire Romain" => PlayerEnum::Roman,
            "Empire Carthaginois" => PlayerEnum::Carthaginois,
            "Empire Grec" => PlayerEnum::Greek,
            "Empire Egyptien" => PlayerEnum::Egypsian,
            _ => continue,
        };
        players_map.insert(player_enum, player.clone());
    }
    
    (players, players_map)
}