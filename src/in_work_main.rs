use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use warp::Filter;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Player {
    id: usize,
    name: String,
    resources: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Move {
    player_id: usize,
    action: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GameState {
    players: Vec<Player>,
    turn: usize,
}

#[tokio::main]
async fn main() {
    let game_state = Arc::new(Mutex::new(GameState {
        players: vec![],
        turn: 0,
    }));

    let get_state = {
        let game_state = Arc::clone(&game_state);
        warp::path("state")
            .and(warp::get())
            .map(move || {
                let state = game_state.lock().unwrap();
                warp::reply::json(&*state)
            })
    };

    let add_player = {
        let game_state = Arc::clone(&game_state);
        warp::path("add_player")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |new_player: Player| {
                let mut state = game_state.lock().unwrap();
                state.players.push(new_player);
                warp::reply::json(&*state)
            })
    };

    let make_move = {
        let game_state = Arc::clone(&game_state);
        warp::path("make_move")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |player_move: Move| {
                let mut state = game_state.lock().unwrap();
                // Simplified move logic for demonstration purposes
                if let Some(player) = state.players.iter_mut().find(|p| p.id == player_move.player_id) {
                    player.resources += 1; // Example action
                    state.turn += 1;
                }
                warp::reply::json(&*state)
            })
    };

    let routes = get_state.or(add_player).or(make_move);

    warp::serve(routes).run(([127.0.0.1], 3030)).await;
}