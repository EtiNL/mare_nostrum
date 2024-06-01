pub mod game_phases {
    #[derive(Debug, Clone)]
    pub enum GamePhases {
        Production,
        Commerce,
        Achats,
        Militaire,
        Revendication
    }

    pub fn phase_transition(phase: GamePhases) -> GamePhases {
        match phase {
            GamePhases::Production => GamePhases::Commerce,
            GamePhases::Commerce => GamePhases::Achats,
            GamePhases::Achats => GamePhases::Militaire,
            GamePhases::Militaire => GamePhases::Revendication,
            GamePhases::Revendication => GamePhases::Production,
        }
    }
}