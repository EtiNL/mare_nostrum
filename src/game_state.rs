use crate::game_phases::game_phases::GamePhases;
use crate::player::{Player, PlayerEnum, new_players};
use crate::plateau::plateau::{Territoire, TerritoireEnum, map_construction, get_territoire};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub mod game_state {
    use super::{GamePhases, Player, Territoire, TerritoireEnum, Arc, HashMap, PlayerEnum, Mutex, map_construction, new_players, get_territoire};
    use std::sync::MutexGuard;

    #[derive(Debug, Clone)]
    pub struct GameState {
        pub current_phase: GamePhases,
        pub players: Vec<Arc<Mutex<Player>>>,
        pub players_hashmap: HashMap<PlayerEnum, Arc<Mutex<Player>>>,
        pub map: Vec<Arc<Mutex<Territoire>>>,
        pub hashmap: HashMap<TerritoireEnum, Arc<Mutex<Territoire>>>,
        pub monopole_leaders: (PlayerEnum, PlayerEnum, PlayerEnum), // Commerce, Politique, Militaire
    }

    impl GameState {
        pub fn get_monopole(&self) {
            for player_ref in &self.players {}
        }

        pub fn print_attributes(&self) {
            println!("GameState {{");
            println!("    current_phase: {:?},", self.current_phase);
            println!("    players: [");
            for player in &self.players {
                let player: MutexGuard<Player> = player.lock().unwrap();
                println!("        Player {{");
                println!("            name: \"{}\",", player.name);
                println!("            heroes_and_wonders: {:?},", player.heroes_and_wonders);
                println!("            ressources: {:?},", player.ressources);
                println!("            territoires: {:?},", player.territoires);
                println!("            abilities: {:?},", player.abilities);
                println!("            monopoles: {:?},", player.monopoles);
                println!("        }},");
            }
            println!("    ],");
            println!("    players_hashmap: {{");
            for (key, player) in &self.players_hashmap {
                let player: MutexGuard<Player> = player.lock().unwrap();
                println!("        {:?}: Player {{", key);
                println!("            name: \"{}\",", player.name);
                println!("            heroes_and_wonders: {:?},", player.heroes_and_wonders);
                println!("            ressources: {:?},", player.ressources);
                println!("            territoires: {:?},", player.territoires);
                println!("            abilities: {:?},", player.abilities);
                println!("            monopoles: {:?},", player.monopoles);
                println!("        }},");
            }
            println!("    }},");
            println!("    map: [");
            for territoire in &self.map {
                let territoire: MutexGuard<Territoire> = territoire.lock().unwrap();
                println!("        Territoire {{");
                println!("            name: \"{}\",", territoire.name);
                println!("            type_terrain: {:?},", territoire.type_terrain);
                println!("            proprietaire: {:?},", territoire.proprietaire);
                println!("            constructions: {:?},", territoire.constructions);
                println!("            neighbours: {:?},", territoire.neighbours);
                println!("            militaire: {:?},", territoire.militaire);
                println!("        }},");
            }
            println!("    ],");
            println!("    hashmap: {{");
            for (key, territoire) in &self.hashmap {
                let territoire: MutexGuard<Territoire> = territoire.lock().unwrap();
                println!("        {:?}: Territoire {{", key);
                println!("            name: \"{}\",", territoire.name);
                println!("            type_terrain: {:?},", territoire.type_terrain);
                println!("            proprietaire: {:?},", territoire.proprietaire);
                println!("            constructions: {:?},", territoire.constructions);
                println!("            neighbours: {:?},", territoire.neighbours);
                println!("            militaire: {:?},", territoire.militaire);
                println!("        }},");
            }
            println!("    }},");
            println!("    monopole_counter: {:?},", self.monopole_leaders);
            println!("}}");
        }

        pub fn update_map_proprietors(&mut self) {
            // Assign proprietors based on player territories
            for player_enum in PlayerEnum::iterator() {
                let player_lock = PlayerEnum::get_player(&self.players_hashmap, &player_enum).unwrap();
                let territoires = player_lock.lock().unwrap().territoires.clone();
                for territoire_enum in territoires {
                    let territoire_lock = get_territoire(&self.hashmap, &territoire_enum).unwrap();
                    territoire_lock.lock().unwrap().proprietaire = Some(player_enum.clone());
                }
            }
        }
    }

    pub fn new_game() -> GameState {
        let (mut _players, _players_map) = new_players();
        let (_map_hashmap, mut _map) = map_construction();
        let mut gamestate = GameState {
            current_phase: GamePhases::Commerce,
            players: _players,
            players_hashmap: _players_map,
            map: _map,
            hashmap: _map_hashmap,
            monopole_leaders: (PlayerEnum::Carthaginois, PlayerEnum::Egypsian, PlayerEnum::Roman),
        };
        gamestate.update_map_proprietors();
        gamestate
    }
}
