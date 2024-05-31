pub mod game_phases {
    #[derive(Debug, Clone)]
    pub enum GamePhases {
        Production,
        Commerce,
        Achats,
        Militaire,
        Revendication
    }
}