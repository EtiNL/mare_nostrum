use crate::game_phases::game_phases::GamePhases;
use crate::player::{Player, PlayerEnum};
use crate::plateau::plateau::{Territoire, TerritoireEnum, map_construction};
use crate::monopole_counter::monopole_counter::Monopole;
use std::sync::Arc;
use std::collections::HashMap;

pub mod game_state {
    use super::{GamePhases, Player, Territoire, Monopole, TerritoireEnum, Arc, HashMap};

    pub struct GameState {
        current_phase: GamePhases,
        players: Vec<Arc<Player>>,
        players_hashmap:HashMap<PlayerEnum, Arc<Player>>,
        map: Vec<Arc<Territoire>>,
        hashmap: HashMap<TerritoireEnum, Arc<Territoire>>,
        monopole_counter: (PlayerEnum, PlayerEnum, PlayerEnum), // Commerce, Politique, Militaire
    }

    impl GameState {
        pub fn new() -> GameState {
            let (_players, _players_map) = Player.new_game();
            let (_map, _map_hashmap) = map_construction();
            GameState {
                current_phase: GamePhases::Commerce,
                players: _players,
                map: _map,
                hashmap: _hashmap,
                monopole_leaders: (PlayerEnum::Carthaginois, PlayerEnum::Egypsian, PlayerEnum::Roman),
            }
        }

        pub fn get_monopole(&self) {
            for player_ref in &self.players {}
        }
    }
}