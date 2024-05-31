mod batiments;
mod plateau;
mod ressources;
mod player;
mod materiel_militaire;
mod heroes_and_wonders;
mod game_phases;
mod score_board;

use crate::plateau::plateau::map_construction;
fn main() {
    let map = map_construction();
    for territoire_ref in &map {
        let name = (territoire_ref.name).clone();
        println!("{name}");
    }
}