pub mod materiel_militaire{
    pub enum ActionsLegions {
        Converti,
        Pille,
        Occupe,
        Deffend
    }

    pub enum MaterielMilitaire {
        Legion(ActionsLegions),
        Trireme,
    }
}