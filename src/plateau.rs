use crate::batiments::batiments::BatimentsTypes;
use crate::ressources::ressources_mod::RessourceType;
use crate::materiel_militaire::materiel_militaire::MaterielMilitaire;
use crate::player::Player;
use std::collections::HashMap;
use std::sync::Arc;

pub mod plateau {
    use super::{BatimentsTypes, MaterielMilitaire, Player, HashMap, Arc, RessourceType};

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
        pub fn new(name: &str, terrain: Terrain, player: Option<&'static Player>, 
                    _constructions: Vec<(BatimentsTypes, bool)>,
                    _neighbours: Vec<TerritoireEnum>, 
                    _militaire: Vec<(&'static Player, MaterielMilitaire)>) -> Territoire {
            Territoire {
                name: name.to_string(),
                type_terrain: terrain,
                proprietaire: player,
                constructions: _constructions,
                neighbours: _neighbours,
                militaire: _militaire,
            }
        }
    }

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
        Aethiopia,
        MareAegyptum,
        Creta,
        Judea,
        Cilicia,
        MareGyprium,
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
                TerritoireEnum::Aethiopia,
                TerritoireEnum::MareAegyptum,
                TerritoireEnum::Creta,
                TerritoireEnum::Judea,
                TerritoireEnum::Cilicia,
                TerritoireEnum::MareGyprium,
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
                TerritoireEnum::Aethiopia => "Aethiopia",
                TerritoireEnum::MareAegyptum => "Mare Aegyptum",
                TerritoireEnum::Creta => "Creta",
                TerritoireEnum::Judea => "Judea",
                TerritoireEnum::Cilicia => "Cilicia",
                TerritoireEnum::MareGyprium => "Mare Gyprium",
                TerritoireEnum::Cappadocia => "Cappadocia",
                TerritoireEnum::PontusEuxinus => "Pontus Euxinus",
                TerritoireEnum::Asia => "Asia",
            }
            .to_string()
        }

        pub fn create(&self) -> Territoire {
            match self {
                TerritoireEnum::Gallia => {Territoire::new("Gallia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Cereales), false),
                                                                                                                            (BatimentsTypes::Caravane(RessourceType::Mouton), false),
                                                                                                                            (BatimentsTypes::Marche, false)], 
                                                                                                        vec![TerritoireEnum::Germania, TerritoireEnum::Dalmatia,
                                                                                                                            TerritoireEnum::Latium, TerritoireEnum::MareTyrrhenum],
                                                                                                        Vec::new())},

                TerritoireEnum::Germania => Territoire::new("Germania", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Diamant), false),
                                                                                                                                (BatimentsTypes::Marche, false)], 
                                                                                                            vec![TerritoireEnum::Gallia, TerritoireEnum::Dalmatia,
                                                                                                                            TerritoireEnum::Dacia],
                                                                                                            Vec::new()),
                                                                                                            
                TerritoireEnum::Dalmatia => Territoire::new("Dalmatia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Bois), false),
                                                                                                                                (BatimentsTypes::Caravane(RessourceType::Gladiateur), false),
                                                                                                                                (BatimentsTypes::Marche, false)], 
                                                                                                            vec![TerritoireEnum::MareHadriaticum, TerritoireEnum::Macedonia,
                                                                                                                            TerritoireEnum::Dacia, TerritoireEnum::Gallia, TerritoireEnum::Germania],
                                                                                                            Vec::new()),

                TerritoireEnum::Latium => Territoire::new("Latium", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Metal), false),
                                                                                                                            (BatimentsTypes::Caravane(RessourceType::Vin), false),
                                                                                                                            (BatimentsTypes::Marche, false),
                                                                                                                            (BatimentsTypes::Capitale, false),
                                                                                                                            (BatimentsTypes::Temple, false)], 
                                                                                                            vec![TerritoireEnum::MareHadriaticum, TerritoireEnum::MareTyrrhenum,
                                                                                                                        TerritoireEnum::Italia, TerritoireEnum::Gallia],
                                                                                                            Vec::new()),

                TerritoireEnum::Italia => Territoire::new("Italia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Metal), false),
                                                                                                                            (BatimentsTypes::Caravane(RessourceType::Huile), false),
                                                                                                                            (BatimentsTypes::Marche, false)], 
                                                                                                            vec![TerritoireEnum::MareHadriaticum, TerritoireEnum::MareTyrrhenum,
                                                                                                                        TerritoireEnum::MareIonium, TerritoireEnum::Latium],
                                                                                                            Vec::new()),

                TerritoireEnum::MareHadriaticum => Territoire::new("Mare Hadriaticum", Terrain::Mer, None, Vec::new(), 
                                                                                                                            vec![TerritoireEnum::MareIonium, TerritoireEnum::Italia,
                                                                                                                                        TerritoireEnum::Macedonia, TerritoireEnum::Latium, 
                                                                                                                                        TerritoireEnum::Dalmatia, TerritoireEnum::Gallia],
                                                                                                                            Vec::new()),

                TerritoireEnum::Macedonia => Territoire::new("Macedonia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Metal), false),
                                                                                                                                (BatimentsTypes::Caravane(RessourceType::Huile), false),
                                                                                                                                (BatimentsTypes::Marche, false)], 
                                                                                                                vec![TerritoireEnum::MareHadriaticum, TerritoireEnum::MareAegaeum,
                                                                                                                            TerritoireEnum::MareIonium, TerritoireEnum::Achea, 
                                                                                                                            TerritoireEnum::Thracia, TerritoireEnum::Dacia, 
                                                                                                                            TerritoireEnum::Germania],
                                                                                                                Vec::new()),

                TerritoireEnum::Dacia => Territoire::new("Dacia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Vin), false),
                                                                                                                        (BatimentsTypes::Caravane(RessourceType::Marbre), false),
                                                                                                                        (BatimentsTypes::Marche, false)], 
                                                                                                        vec![TerritoireEnum::PontusEuxinus, TerritoireEnum::Dalmatia, 
                                                                                                                    TerritoireEnum::Thracia, TerritoireEnum::Macedonia, 
                                                                                                                    TerritoireEnum::Germania],
                                                                                                        Vec::new()),

                TerritoireEnum::Thracia => Territoire::new("Thracia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Marbre), false),
                                                                                                                            (BatimentsTypes::Marche, false),
                                                                                                                            (BatimentsTypes::Cite, false),
                                                                                                                            (BatimentsTypes::Temple, false)], 
                                                                                                            vec![TerritoireEnum::PontusEuxinus, TerritoireEnum::Dacia, 
                                                                                                                        TerritoireEnum::MareAegaeum, TerritoireEnum::Macedonia],
                                                                                                            Vec::new()),

                TerritoireEnum::Achea => Territoire::new("Achea", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Huile), false),
                                                                                                                        (BatimentsTypes::Marche, false),
                                                                                                                        (BatimentsTypes::Capitale, false),
                                                                                                                        (BatimentsTypes::Cite, false),
                                                                                                                        (BatimentsTypes::Temple, false)], 
                                                                                                        vec![TerritoireEnum::MareIonium, TerritoireEnum::MareAegaeum, 
                                                                                                                        TerritoireEnum::Macedonia],
                                                                                                        Vec::new()),

                TerritoireEnum::MareIonium => Territoire::new("Mare Ionium", Terrain::Mer, None, Vec::new(), 
                                                                                                                vec![TerritoireEnum::Sicilia, TerritoireEnum::MareHadriaticum, 
                                                                                                                                TerritoireEnum::Macedonia, TerritoireEnum::MareTyrrhenum,
                                                                                                                                TerritoireEnum::MareAfricum, TerritoireEnum::MareAegaeum,
                                                                                                                                TerritoireEnum::Achea, TerritoireEnum::Italia],
                                                                                                                Vec::new()),

                TerritoireEnum::MareAegaeum => Territoire::new("Mare Aegaeum", Terrain::Mer, None, Vec::new(), 
                                                                                                                vec![TerritoireEnum::Creta, TerritoireEnum::MareIonium, 
                                                                                                                                TerritoireEnum::Macedonia, TerritoireEnum::PontusEuxinus,
                                                                                                                                TerritoireEnum::MareAfricum, TerritoireEnum::MareAegyptum,
                                                                                                                                TerritoireEnum::Achea, TerritoireEnum::Thracia,
                                                                                                                                TerritoireEnum::Asia, TerritoireEnum::MareGyprium],
                                                                                                                Vec::new()),

                TerritoireEnum::MareTyrrhenum => Territoire::new("Mare Tyrrhenum", Terrain::Mer, None, Vec::new(), 
                                                                                                                    vec![TerritoireEnum::Sicilia, TerritoireEnum::MareIonium, 
                                                                                                                                    TerritoireEnum::MareSardum, TerritoireEnum::Italia,
                                                                                                                                    TerritoireEnum::Latium, TerritoireEnum::Gallia],
                                                                                                                    Vec::new()),

                TerritoireEnum::MareSardum => Territoire::new("Mare Sardum", Terrain::Mer, None, Vec::new(), 
                                                                                                                vec![TerritoireEnum::Sicilia, TerritoireEnum::Africa, 
                                                                                                                                TerritoireEnum::MareTyrrhenum, TerritoireEnum::Numidia,
                                                                                                                                TerritoireEnum::MareAfricum],
                                                                                                                Vec::new()),

                TerritoireEnum::MareAfricum => Territoire::new("Mare Africum", Terrain::Mer, None, Vec::new(), 
                                                                                                                    vec![TerritoireEnum::Sicilia, TerritoireEnum::Africa, 
                                                                                                                                    TerritoireEnum::Libya, TerritoireEnum::Cyrenaica,
                                                                                                                                    TerritoireEnum::MareSardum, TerritoireEnum::MareAegyptum,
                                                                                                                                    TerritoireEnum::Creta, TerritoireEnum::MareAegaeum,
                                                                                                                                    TerritoireEnum::MareIonium],
                                                                                                                    Vec::new()),

                TerritoireEnum::Sicilia => Territoire::new("Sicilia", Terrain::Terre, None, vec![(BatimentsTypes::CiteLegendaire, false),
                                                                                                                            (BatimentsTypes::Temple, false)], 
                                                                                                            vec![TerritoireEnum::MareIonium, TerritoireEnum::MareAfricum, 
                                                                                                                            TerritoireEnum::MareTyrrhenum, TerritoireEnum::MareSardum],
                                                                                                            Vec::new()),

                TerritoireEnum::Numidia => Territoire::new("Numidia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Or), false),
                                                                                                                            (BatimentsTypes::Caravane(RessourceType::Gladiateur), false),
                                                                                                                            (BatimentsTypes::Marche, false)], 
                                                                                                            vec![TerritoireEnum::Africa, TerritoireEnum::Phasania,
                                                                                                                             TerritoireEnum::MareSardum],
                                                                                                            Vec::new()),

                TerritoireEnum::Africa => Territoire::new("Africa", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Mouton), false),
                                                                                                                        (BatimentsTypes::Caravane(RessourceType::Parfum), false),
                                                                                                                        (BatimentsTypes::Marche, false)], 
                                                                                                        vec![TerritoireEnum::Numidia, TerritoireEnum::Phasania,
                                                                                                                        TerritoireEnum::MareSardum, TerritoireEnum::MareAfricum,
                                                                                                                        TerritoireEnum::Libya],
                                                                                                        Vec::new()),

                TerritoireEnum::Phasania => Territoire::new("Phasania", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Gladiateur), false),
                                                                                                                            (BatimentsTypes::Marche, false)], 
                                                                                                            vec![TerritoireEnum::Numidia, TerritoireEnum::Garamantia,
                                                                                                                            TerritoireEnum::Africa, TerritoireEnum::Libya],
                                                                                                            Vec::new()),

                TerritoireEnum::Libya => Territoire::new("Libya", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Huile), false),
                                                                                                                        (BatimentsTypes::Caravane(RessourceType::Mouton), false),
                                                                                                                        (BatimentsTypes::Marche, false)], 
                                                                                                        vec![TerritoireEnum::Phasania, TerritoireEnum::Garamantia,
                                                                                                                        TerritoireEnum::Africa, TerritoireEnum::MareAfricum,
                                                                                                                        TerritoireEnum::Cyrenaica],
                                                                                                        Vec::new()),

                TerritoireEnum::Garamantia => Territoire::new("Garamantia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Huile), false),
                                                                                                                                    (BatimentsTypes::Caravane(RessourceType::Diamant), false),
                                                                                                                                    (BatimentsTypes::Marche, false)], 
                                                                                                                    vec![TerritoireEnum::Phasania, TerritoireEnum::Libya,
                                                                                                                                    TerritoireEnum::Nubia, TerritoireEnum::Cyrenaica],
                                                                                                                    Vec::new()),

                TerritoireEnum::Cyrenaica => Territoire::new("Cyrenaica", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Papyrus), false),
                                                                                                                                (BatimentsTypes::Marche, false),
                                                                                                                                (BatimentsTypes::Cite, false),
                                                                                                                                (BatimentsTypes::Temple, false)], 
                                                                                                                vec![TerritoireEnum::Libya, TerritoireEnum::Garamantia,
                                                                                                                                TerritoireEnum::Nubia, TerritoireEnum::MareAfricum,
                                                                                                                                TerritoireEnum::MareAegyptum, TerritoireEnum::Aegyptus],
                                                                                                                Vec::new()),

                TerritoireEnum::Nubia => Territoire::new("Nubia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Gladiateur), false),
                                                                                                                        (BatimentsTypes::Marche, false)], 
                                                                                                        vec![TerritoireEnum::Cyrenaica, TerritoireEnum::Garamantia,
                                                                                                                        TerritoireEnum::Aethiopia, TerritoireEnum::Aegyptus],
                                                                                                        Vec::new()),

                TerritoireEnum::Aegyptus => Territoire::new("Aegyptus", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Papyrus), false),
                                                                                                                            (BatimentsTypes::Marche, false),
                                                                                                                            (BatimentsTypes::Capitale, false),
                                                                                                                            (BatimentsTypes::Cite, false),
                                                                                                                            (BatimentsTypes::Temple, false)], 
                                                                                                            vec![TerritoireEnum::Cyrenaica, TerritoireEnum::Judea,
                                                                                                                            TerritoireEnum::Aethiopia, TerritoireEnum::MareAegyptum,
                                                                                                                            TerritoireEnum::Nubia],
                                                                                                            Vec::new()),

                TerritoireEnum::Aethiopia => Territoire::new("Aethiopia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Cereales), false),
                                                                                                                            (BatimentsTypes::Caravane(RessourceType::Or), false),
                                                                                                                            (BatimentsTypes::Marche, false)], 
                                                                                                            vec![TerritoireEnum::Aegyptus, TerritoireEnum::Nubia],
                                                                                                            Vec::new()),

                TerritoireEnum::MareAegyptum => Territoire::new("Mare Aegyptum", Terrain::Mer, None, Vec::new(), 
                                                                                                                    vec![TerritoireEnum::Cyrenaica, TerritoireEnum::Aegyptus, 
                                                                                                                                    TerritoireEnum::Judea, TerritoireEnum::MareGyprium,
                                                                                                                                    TerritoireEnum::MareAegaeum, TerritoireEnum::Creta,
                                                                                                                                    TerritoireEnum::MareAfricum],
                                                                                                                    Vec::new()),

                TerritoireEnum::Creta => Territoire::new("Creta", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Ceramique), false),
                                                                                                                        (BatimentsTypes::Marche, false),
                                                                                                                        (BatimentsTypes::Cite, false),
                                                                                                                        (BatimentsTypes::Temple, false)], 
                                                                                                        vec![TerritoireEnum::MareAegaeum, TerritoireEnum::MareAfricum,
                                                                                                                        TerritoireEnum::MareAegyptum],
                                                                                                        Vec::new()),

                TerritoireEnum::Judea => Territoire::new("Judea", Terrain::Terre, None, vec![(BatimentsTypes::CiteLegendaire, false),
                                                                                                                        (BatimentsTypes::Temple, false)], 
                                                                                                        vec![TerritoireEnum::Aegyptus, TerritoireEnum::MareGyprium,
                                                                                                                        TerritoireEnum::MareAegyptum, TerritoireEnum::Cilicia],
                                                                                                        Vec::new()),

                TerritoireEnum::Cilicia => Territoire::new("Cilicia", Terrain::Terre, None, vec![(BatimentsTypes::Capitale, false),
                                                                                                                            (BatimentsTypes::Temple, false)], 
                                                                                                            vec![TerritoireEnum::Judea, TerritoireEnum::MareGyprium,
                                                                                                                            TerritoireEnum::Cappadocia, TerritoireEnum::Asia],
                                                                                                            Vec::new()),

                TerritoireEnum::MareGyprium => Territoire::new("Mare Gyprium", Terrain::Mer, None, Vec::new(), 
                                                                                                                    vec![TerritoireEnum::Cilicia, TerritoireEnum::Asia, 
                                                                                                                                    TerritoireEnum::Judea, TerritoireEnum::MareAegyptum,
                                                                                                                                    TerritoireEnum::MareAegaeum],
                                                                                                                    Vec::new()),

                TerritoireEnum::Cappadocia => Territoire::new("Cappadocia", Terrain::Terre, None, vec![(BatimentsTypes::Caravane(RessourceType::Bois), false),
                                                                                                                                   (BatimentsTypes::Marche, false)], 
                                                                                                                vec![TerritoireEnum::Cilicia, TerritoireEnum::PontusEuxinus,
                                                                                                                                TerritoireEnum::Asia],
                                                                                                                Vec::new()),

                TerritoireEnum::PontusEuxinus => Territoire::new("Pontus Euxinus", Terrain::Mer, None, Vec::new(), 
                                                                                                                        vec![TerritoireEnum::Cappadocia, TerritoireEnum::Asia, 
                                                                                                                                        TerritoireEnum::MareAegaeum, TerritoireEnum::Thracia,
                                                                                                                                        TerritoireEnum::Dacia],
                                                                                                                        Vec::new()),
                TerritoireEnum::Asia => Territoire::new("Asia", Terrain::Terre, None, vec![(BatimentsTypes::CiteLegendaire, false),
                                                                                                                    (BatimentsTypes::Temple, false)], 
                                                                                                    vec![TerritoireEnum::Cilicia, TerritoireEnum::PontusEuxinus,
                                                                                                                TerritoireEnum::Cappadocia, TerritoireEnum::MareGyprium,
                                                                                                                TerritoireEnum::MareAegaeum],
                                                                                                    Vec::new()),
            }
        }
    }

    pub fn map_construction() ->  (HashMap<String, Arc<Territoire>>,Vec<Arc<Territoire>>) {
        let mut map = Vec::new();
        let mut hashmap = HashMap::new();
        
        for territoire in TerritoireEnum::iterator() {
            let created_territoire = Arc::new(territoire.create());
            map.push(Arc::clone(&created_territoire));
            hashmap.insert(territoire.to_string(), created_territoire);
        }

        (hashmap, map)
    }

    // Function to get a reference to the corresponding `Territoire` from the map
    pub fn get_territoire(hashmap: &HashMap<String, Arc<Territoire>>, territoire_enum: &TerritoireEnum) -> Option<Arc<Territoire>> {
        hashmap.get(&territoire_enum.to_string()).cloned()
    }

    // Example usage
    pub fn test_get_territoire() {
        let (hashmap, _map) = map_construction();
        let territoire_enum = TerritoireEnum::Gallia;
        
        if let Some(territoire) = get_territoire(&hashmap, &territoire_enum) {
            println!("{:?}", territoire);
        } else {
            println!("Territoire not found");
        }
    }
}
