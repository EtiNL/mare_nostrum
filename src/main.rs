mod batiments;
mod plateau;
mod ressources;
mod player;
mod materiel_militaire;
mod heroes_and_wonders;
mod game_phases;
mod monopole_counter;
mod game_state;

use crate::game_state::game_state::new_game;
fn main() {
    let mut game_state = new_game();
    game_state.print_attributes();
}