use crate::game_phases::game_phases::GamePhases;
use crate::player::Player;
use crate::plateau::plateau::Territoire;
use crate::monopole_counter::monopole_counter::Monopole;

pub mod game_state {
    use super::{GamePhases, Player, Territoire, Monopole};

    pub struct GameState {
        current_phase: GamePhases,
        players: Vec<Player>,
        map: Vec<Arc<Territoire>>,
        hashmap: HashMap<String, Arc<Territoire>>,
        monopole_counter: Vec<(&Player, Monopole)>,
    }

    impl GameState {
        pub fn new(_players: Vec<Player>, _map:Vec<Arc<Territoire>>, _hashmap: HashMap<String, Arc<Territoire>>,) -> GameState {
            GameState {
                current_phase: GamePhases::Commerce,
                players: _players,
                map: _map,
                hashmap: _hashmap,
            }
        }

        pub fn get_monopole(&self) {
            for player_ref in &self.players {}
        }
    }
}