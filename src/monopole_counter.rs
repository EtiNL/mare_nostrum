pub mod monopole_counter {

    #[derive(Debug, Clone)]
    pub enum Monopole {
        Militaire(u8),
        Politique(u8),
        Commercial(u8)
    }
}