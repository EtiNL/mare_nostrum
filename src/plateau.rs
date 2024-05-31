
use batiments::BatimentsTypes;
use materiel_militaire::MaterielMilitaire;
use player::Player;

pub mod plateau {
    enum Terrain {
        Terre,
        Mer,
    }

    struct Territoire {
        name: String,
        type_terrain: Terrain,
        proprietaire: Option<&Player>,
        constructions: Vect<(BatimentsTypes,bool)>,
        neighbours: Vect<Territoire>,
        militaire: Vect<(&Player, MaterielMilitaire)>,
    }

    impl Territoire {
        pub fn new(name: &str, terrain: Terrain, player: Option<&Player>) -> Territoire {
            Territoire {
                name: name.to_string(),
                type_terrain: terrain,
                proprietaire: player,
                constructions: Vect::new(),
                neighbours: Vect::new(),
                militaire: Vect::new(),
            }
        }
    }

    pub enum TerritoireEnum {
        Gallia,
        Germania,
        Dalmatia,
        Latium,
        Italia,
        MareHadriaticum,
        Macedonia,
        Dacia,
        Thracia,
        Achea,
        MareIonium,
        MareAegaeum,
        MareTyrrhenum,
        MareSardum,
        MareAfricum,
        Sicilia,
        Numidia,
        Africa,
        Phasania,
        Libya,
        Garamantia,
        Cyrenaica,
        Nubia,
        Aegyptus,
        Aetiopia,
        MareAegyptum,
        Creta,
        Judea,
        Cilicia,
        MareCyprium,
        Cappadocia,
        PontusEuxinus,
        Asia
    }

    impl TerritoireEnum {
        pub fn iterator() -> impl Iterator<Item = TerritoireEnum> {
            [
                TerritoireEnum::Gallia,
                TerritoireEnum::Germania,
                TerritoireEnum::Dalmatia,
                TerritoireEnum::Latium,
                TerritoireEnum::Italia,
                TerritoireEnum::MareHadriaticum,
                TerritoireEnum::Macedonia,
                TerritoireEnum::Dacia,
                TerritoireEnum::Thracia,
                TerritoireEnum::Achea,
                TerritoireEnum::MareIonium,
                TerritoireEnum::MareAegaeum,
                TerritoireEnum::MareTyrrhenum,
                TerritoireEnum::MareSardum,
                TerritoireEnum::MareAfricum,
                TerritoireEnum::Sicilia,
                TerritoireEnum::Numidia,
                TerritoireEnum::Africa,
                TerritoireEnum::Phasania,
                TerritoireEnum::Libya,
                TerritoireEnum::Garamantia,
                TerritoireEnum::Cyrenaica,
                TerritoireEnum::Nubia,
                TerritoireEnum::Aegyptus,
                TerritoireEnum::Aetiopia,
                TerritoireEnum::MareAegyptum,
                TerritoireEnum::Creta,
                TerritoireEnum::Judea,
                TerritoireEnum::Cilicia,
                TerritoireEnum::MareCyprium,
                TerritoireEnum::Cappadocia,
                TerritoireEnum::PontusEuxinus,
                TerritoireEnum::Asia
            ]
            .iter()
            .cloned()
        }

        pub fn map_construction() -> Vect<Territoire>{
            let map = Vect::new();
            for territoire in TerritoireEnum::iterator() {
                map.push(territoire.create());
            }
            map
        }
        
        pub fn create(&self) -> Territoire {
            match self {
                TerritoireEnum::Gallia => Territoire::new(
                    "Gallia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Germania => Territoire::new(
                    "Germania",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Dalmatia => Territoire::new(
                    "Dalmatia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Latium => Territoire::new(
                    "Latium",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Italia => Territoire::new(
                    "Italia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::MareHadriaticum => Territoire::new(
                    "Mare Hadriaticum",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::Macedonia => Territoire::new(
                    "Macedonia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Dacia => Territoire::new(
                    "Dacia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Thracia => Territoire::new(
                    "Thracia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Achea => Territoire::new(
                    "Achea",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::MareIonium => Territoire::new(
                    "Mare Ionium",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::MareAegaeum => Territoire::new(
                    "Mare Aegaeum",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::MareTyrrhenum => Territoire::new(
                    "Mare Tyrrhenum",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::MareSardum => Territoire::new(
                    "Mare Sardum",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::MareAfricum => Territoire::new(
                    "Mare Africum",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::Sicilia => Territoire::new(
                    "Sicilia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Numidia => Territoire::new(
                    "Numidia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Africa => Territoire::new(
                    "Africa",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Phasania => Territoire::new(
                    "Phasania",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Libya => Territoire::new(
                    "Libya",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Garamantia => Territoire::new(
                    "Garamantia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Cyrenaica => Territoire::new(
                    "Cyrenaica",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Nubia => Territoire::new(
                    "Nubia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Aegyptus => Territoire::new(
                    "Aegyptus",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Aetiopia => Territoire::new(
                    "Aetiopia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::MareAegyptum => Territoire::new(
                    "Mare Aegyptum",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::Creta => Territoire::new(
                    "Creta",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Judea => Territoire::new(
                    "Judea",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::Cilicia => Territoire::new(
                    "Cilicia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::MareCyprium => Territoire::new(
                    "Mare Cyprium",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::Cappadocia => Territoire::new(
                    "Cappadocia",
                    Terrain::Terre,
                    None,
                ),
                TerritoireEnum::PontusEuxinus => Territoire::new(
                    "Pontus Euxinus",
                    Terrain::Mer,
                    None,
                ),
                TerritoireEnum::Asia => Territoire::new(
                    "Asia",
                    Terrain::Terre,
                    None,
                ),
            }
        }
    }
}