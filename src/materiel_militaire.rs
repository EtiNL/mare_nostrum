pub mod materiel_militaire{
    #[derive(Debug, Clone)]
    pub enum ActionsLegions {
        Converti,
        Pille,
        Occupe,
        Deffend
    }

    #[derive(Debug, Clone)]
    pub enum MaterielMilitaire {
        Legion(ActionsLegions),
        Trireme,
    }
}