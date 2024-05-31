use crate::batiments::batiments::BatimentsTypes;
use crate::materiel_militaire::materiel_militaire::MaterielMilitaire;
use crate::player::Player;
use std::collections::HashMap;

pub mod plateau {
    use super::{BatimentsTypes, MaterielMilitaire, Player, HashMap};

    #[derive(Debug, Clone)]
    pub enum Terrain {
        Terre,
        Mer,
    }

    #[derive(Debug, Clone)]
    pub struct Territoire {
        pub name: String,
        pub type_terrain: Terrain,
        pub proprietaire: Option<&'static Player>,
        pub constructions: Vec<(BatimentsTypes, bool)>,
        pub neighbours: Vec<TerritoireEnum>,
        pub militaire: Vec<(&'static Player, MaterielMilitaire)>,
    }

    impl Territoire {
        pub fn new(name: &str, terrain: Terrain, player: Option<&'static Player>) -> Territoire {
            Territoire {
                name: name.to_string(),
                type_terrain: terrain,
                proprietaire: player,
                constructions: Vec::new(),
                neighbours: Vec::new(),
                militaire: Vec::new(),
            }
        }
    }

    #[derive(Debug, Clone)]
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
        Asia,
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
                TerritoireEnum::Asia,
            ]
            .iter()
            .cloned()
        }

        pub fn to_string(&self) -> String {
            match self {
                TerritoireEnum::Gallia => "Gallia",
                TerritoireEnum::Germania => "Germania",
                TerritoireEnum::Dalmatia => "Dalmatia",
                TerritoireEnum::Latium => "Latium",
                TerritoireEnum::Italia => "Italia",
                TerritoireEnum::MareHadriaticum => "Mare Hadriaticum",
                TerritoireEnum::Macedonia => "Macedonia",
                TerritoireEnum::Dacia => "Dacia",
                TerritoireEnum::Thracia => "Thracia",
                TerritoireEnum::Achea => "Achea",
                TerritoireEnum::MareIonium => "Mare Ionium",
                TerritoireEnum::MareAegaeum => "Mare Aegaeum",
                TerritoireEnum::MareTyrrhenum => "Mare Tyrrhenum",
                TerritoireEnum::MareSardum => "Mare Sardum",
                TerritoireEnum::MareAfricum => "Mare Africum",
                TerritoireEnum::Sicilia => "Sicilia",
                TerritoireEnum::Numidia => "Numidia",
                TerritoireEnum::Africa => "Africa",
                TerritoireEnum::Phasania => "Phasania",
                TerritoireEnum::Libya => "Libya",
                TerritoireEnum::Garamantia => "Garamantia",
                TerritoireEnum::Cyrenaica => "Cyrenaica",
                TerritoireEnum::Nubia => "Nubia",
                TerritoireEnum::Aegyptus => "Aegyptus",
                TerritoireEnum::Aetiopia => "Aetiopia",
                TerritoireEnum::MareAegyptum => "Mare Aegyptum",
                TerritoireEnum::Creta => "Creta",
                TerritoireEnum::Judea => "Judea",
                TerritoireEnum::Cilicia => "Cilicia",
                TerritoireEnum::MareCyprium => "Mare Cyprium",
                TerritoireEnum::Cappadocia => "Cappadocia",
                TerritoireEnum::PontusEuxinus => "Pontus Euxinus",
                TerritoireEnum::Asia => "Asia",
            }
            .to_string()
        }

        pub fn create(&self) -> Territoire {
            match self {
                TerritoireEnum::Gallia => Territoire::new("Gallia", Terrain::Terre, None),
                TerritoireEnum::Germania => Territoire::new("Germania", Terrain::Terre, None),
                TerritoireEnum::Dalmatia => Territoire::new("Dalmatia", Terrain::Terre, None),
                TerritoireEnum::Latium => Territoire::new("Latium", Terrain::Terre, None),
                TerritoireEnum::Italia => Territoire::new("Italia", Terrain::Terre, None),
                TerritoireEnum::MareHadriaticum => Territoire::new("Mare Hadriaticum", Terrain::Mer, None),
                TerritoireEnum::Macedonia => Territoire::new("Macedonia", Terrain::Terre, None),
                TerritoireEnum::Dacia => Territoire::new("Dacia", Terrain::Terre, None),
                TerritoireEnum::Thracia => Territoire::new("Thracia", Terrain::Terre, None),
                TerritoireEnum::Achea => Territoire::new("Achea", Terrain::Terre, None),
                TerritoireEnum::MareIonium => Territoire::new("Mare Ionium", Terrain::Mer, None),
                TerritoireEnum::MareAegaeum => Territoire::new("Mare Aegaeum", Terrain::Mer, None),
                TerritoireEnum::MareTyrrhenum => Territoire::new("Mare Tyrrhenum", Terrain::Mer, None),
                TerritoireEnum::MareSardum => Territoire::new("Mare Sardum", Terrain::Mer, None),
                TerritoireEnum::MareAfricum => Territoire::new("Mare Africum", Terrain::Mer, None),
                TerritoireEnum::Sicilia => Territoire::new("Sicilia", Terrain::Terre, None),
                TerritoireEnum::Numidia => Territoire::new("Numidia", Terrain::Terre, None),
                TerritoireEnum::Africa => Territoire::new("Africa", Terrain::Terre, None),
                TerritoireEnum::Phasania => Territoire::new("Phasania", Terrain::Terre, None),
                TerritoireEnum::Libya => Territoire::new("Libya", Terrain::Terre, None),
                TerritoireEnum::Garamantia => Territoire::new("Garamantia", Terrain::Terre, None),
                TerritoireEnum::Cyrenaica => Territoire::new("Cyrenaica", Terrain::Terre, None),
                TerritoireEnum::Nubia => Territoire::new("Nubia", Terrain::Terre, None),
                TerritoireEnum::Aegyptus => Territoire::new("Aegyptus", Terrain::Terre, None),
                TerritoireEnum::Aetiopia => Territoire::new("Aetiopia", Terrain::Terre, None),
                TerritoireEnum::MareAegyptum => Territoire::new("Mare Aegyptum", Terrain::Mer, None),
                TerritoireEnum::Creta => Territoire::new("Creta", Terrain::Terre, None),
                TerritoireEnum::Judea => Territoire::new("Judea", Terrain::Terre, None),
                TerritoireEnum::Cilicia => Territoire::new("Cilicia", Terrain::Terre, None),
                TerritoireEnum::MareCyprium => Territoire::new("Mare Cyprium", Terrain::Mer, None),
                TerritoireEnum::Cappadocia => Territoire::new("Cappadocia", Terrain::Terre, None),
                TerritoireEnum::PontusEuxinus => Territoire::new("Pontus Euxinus", Terrain::Mer, None),
                TerritoireEnum::Asia => Territoire::new("Asia", Terrain::Terre, None),
            }
        }
    }

    pub fn map_construction() -> Vec<Territoire> {
        let mut map = Vec::new();
        for territoire in TerritoireEnum::iterator() {
            map.push(territoire.create());
        }
        for territoire_ref in &mut map {
            if *territoire_ref.name == "Galia".to_string() {
                
            }
        }
        map
    }

    // Constructs a map from `TerritoireEnum` variants to `Territoire` instances
    pub fn create_hashmap() -> HashMap<String, Territoire> {
        let mut map = HashMap::new();
        for territoire in TerritoireEnum::iterator() {
            let territoire_instance = territoire.create();
            map.insert(territoire.to_string(), territoire_instance);
        }
        map
    }

    // Function to get a reference to the corresponding `Territoire` from the map
    pub fn get_territoire<'a>(map: &'a HashMap<String, Territoire>, territoire_enum: &TerritoireEnum) -> Option<&'a Territoire> {
        map.get(&territoire_enum.to_string())
    }

    // Example usage
    pub fn test_get_territoire() {
        let hashmap = create_hashmap();
        let territoire_enum = TerritoireEnum::Gallia;
        
        if let Some(territoire) = get_territoire(&hashmap, &territoire_enum) {
            println!("{:?}", territoire);
        } else {
            println!("Territoire not found");
        }
    }
}
