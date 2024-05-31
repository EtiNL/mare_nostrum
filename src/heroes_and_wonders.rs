use serde::{Deserialize, Serialize};
use score_board::Monopole;
use game_phases::Game_phases;
use plateau::Terrain;

pub mod Heores_and_wonders_mod {
    pub enum Ability {
        AttackDice(Game_phases, Terrain, u8),
        IncludeTaxTokenInGoodCombination(Game_phases),
        IncludeGoodTokenInTaxCombination(Game_phases),
        FreeInfluenceMarker(Game_phases),
        SameGoodTokenInGoodCombination(Game_phases),
        DefenceDice(Game_phases, Terrain, u8),
        ExengeWithTheNumberOfGoodPlayerWants(Game_phases),
        BonusMonopole(Game_phases, Monopole, u8),
        CopyAbility,
        UseLegionAsCaravane(Game_phases),
        ContructForteresseForPierreOrImpot(Game_phases),
        DoubleBeneficePillage(Game_phases),
        Plus5InBattleIfOwnTerritory(Game_phases), // if there is a legion
        FreeLegionInOccupiedTerritory(Game_phases),
        PlayFirstForTheWin(Game_phases),
        OneMoreDice(Game_phases),
        AutomaticallyConvertTerritoryIfLeggionOnInfluenceMarker(Game_phases),
        LegionForGladiatorOrOneTax(Game_phases),
        TakeOneGoodThatPlayerWantsAfterRessourceCollection(Game_phases),
        BuildOneBoatForOneWoodOrOneTax(Game_phases),
        OneTaxFree(Game_phases),
        KeepTwoDifferentGood(Game_phases),
        ChoosePeaceWithAnotherPlayer(Game_phases),
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct HeroAndWonder {
        pub name: String,
        pub abilities: Vec<Ability>,
        pub starting_hero: bool,
    }

    impl HeroAndWonder {
        pub fn new(name: &str, hw_abilities: Vec<Ability>, starter: bool) -> HeroAndWonder {
            HeroAndWonder {
                name: name.to_string(),
                abilities: hw_abilities,
                starting_hero: starter,
            }
        }
    }

    pub enum Heroes_and_Wonders_enum {
        Cesar,
        Cleopatre,
        Hammourabi,
        Hannibal,
        Pericles,
        Antigone,
        CastorEtPolux,
        Circe,
        Gilgamesh,
        Hamilcar,
        Hercule,
        Nabuchodonosor,
        Pentesilee,
        Persee,
        RamsesII,
        ReineDeSaba,
        Spartacus,
        Colosse,
        TempleArtemis,
        Phare,
        StatueDeZeus,
        JardinSuspendus
    }

    impl Heroes_and_Wonders_enum {
        pub fn get_hero_and_wonder(&self) -> HeroAndWonder {
            match self {
                Heroes_and_Wonders_enum::Cesar => HeroAndWonder::new(
                    "Cesar",
                    vec![Ability::AttackDice(Game_phases::militaire, Terrain::terrestre, 1)],
                    true,
                ),
                Heroes_and_Wonders_enum::Cleopatre => HeroAndWonder::new(
                    "Cleopatre",
                    vec![Ability::IncludeTaxTokenInGoodCombination(Game_phases::achat), IncludeGoodTokenInTaxCombination(Game_phases::achat)],
                    true,
                ),
                Heroes_and_Wonders_enum::Hammourabi => HeroAndWonder::new(
                    "Hammourabi",
                    vec![FreeInfluenceMarker(Game_phases::achat)],
                    true,
                ),
                Heroes_and_Wonders_enum::Hannibal => HeroAndWonder::new(
                    "Hannibal",
                    vec![SameGoodTokenInGoodCombination(Game_phases::achat)],
                    true,
                ),
                Heroes_and_Wonders_enum::Pericles => HeroAndWonder::new(
                    "Pericles",
                    vec![DefenceDice(Game_phases::militaire, Terrain::terrestre, 2)],
                    true,
                ),
                Heroes_and_Wonders_enum::Antigone => HeroAndWonder::new(
                    "Antigone",
                    vec![ExengeWithTheNumberOfGoodPlayerWants(Game_phases::commerce),
                        BonusMonopole(Game_phases::revendication, Monopole::commerce, 2),
                        BonusMonopole(Game_phases::revendication, Monopole::politique, 2)],
                    false,
                ),
                Heroes_and_Wonders_enum::CastorEtPolux => HeroAndWonder::new(
                    "Castor Et Polux",
                    vec![CopyAbility],
                    false,
                ),
                Heroes_and_Wonders_enum::Circe => HeroAndWonder::new(
                    "Circé",
                    vec![UseLegionAsCaravane(Game_phases::militaire),
                        BonusMonopole(Game_phases::revendication, Monopole::commerce, 1),
                        BonusMonopole(Game_phases::revendication, Monopole::militaire, 1)],
                    false,
                ),
                Heroes_and_Wonders_enum::Gilgamesh => HeroAndWonder::new(
                    "Gilgamesh",
                    vec![ContructForteresseForPierreOrImpot(Game_phases::achat)],
                    false,
                ),
                Heroes_and_Wonders_enum::Hamilcar => HeroAndWonder::new(
                    "Hamilcar",
                    vec![DoubleBeneficePillage(Game_phases::militaire),
                        BonusMonopole(Game_phases::revendication, Monopole::commerce, 1),
                        BonusMonopole(Game_phases::revendication, Monopole::politique, 1),
                        BonusMonopole(Game_phases::revendication, Monopole::militaire, 1)],
                    false,
                ),
                Heroes_and_Wonders_enum::Hercule => HeroAndWonder::new(
                    "Hercule",
                    vec![BonusMonopole(Game_phases::revendication, Monopole::commerce, 2),
                        BonusMonopole(Game_phases::revendication, Monopole::politique, 2),
                        BonusMonopole(Game_phases::revendication, Monopole::militaire, 2)],
                    false,
                ),
                Heroes_and_Wonders_enum::Nabuchodonosor => HeroAndWonder::new(
                    "Nabuchodonosor",
                    vec![Plus5InBattleIfOwnTerritory(Game_phases::militaire)],
                    false,
                ),
                Heroes_and_Wonders_enum::Pentesilee => HeroAndWonder::new(
                    "Pentesilée",
                    vec![FreeLegionInOccupiedTerritory(Game_phases::achat),
                        BonusMonopole(Game_phases::revendication, Monopole::politique, 2),
                        BonusMonopole(Game_phases::revendication, Monopole::militaire, 2)],
                    false,
                ),
                Heroes_and_Wonders_enum::Persee => HeroAndWonder::new(
                    "Persée",
                    vec![PlayFirstForTheWin(Game_phases::achat),
                        BonusMonopole(Game_phases::revendication, Monopole::commerce, 1),
                        BonusMonopole(Game_phases::revendication, Monopole::politique, 2),
                        BonusMonopole(Game_phases::revendication, Monopole::militaire, 1)],
                    false,
                ),
                Heroes_and_Wonders_enum::RamsesII => HeroAndWonder::new(
                    "Ramses II",
                    vec![OneMoreDice(Game_phases::militaire)],
                    false,
                ),
                Heroes_and_Wonders_enum::ReineDeSaba => HeroAndWonder::new(
                    "Reine de Saba",
                    vec![AutomaticallyConvertTerritoryIfLeggionOnInfluenceMarker(Game_phases::militaire),
                        BonusMonopole(Game_phases::revendication, Monopole::politique, 1),
                        BonusMonopole(Game_phases::revendication, Monopole::militaire, 1)],
                    false,
                ),
                Heroes_and_Wonders_enum::Spartacus => HeroAndWonder::new(
                    "Spartacus",
                    vec![LegionForGladiatorOrOneTax(Game_phases::achat)],
                    false,
                ),
                Heroes_and_Wonders_enum::Colosse => HeroAndWonder::new(
                    "Colosse",
                    vec![TakeOneGoodThatPlayerWantsAfterRessourceCollection(Game_phases::production)],
                    false,
                ),
                Heroes_and_Wonders_enum::TempleArtemis => HeroAndWonder::new(
                    "Temple d'Artémis",
                    vec![OneTaxFree(Game_phases::production)],
                    false,
                ),
                Heroes_and_Wonders_enum::Phare => HeroAndWonder::new(
                    "Phare",
                    vec![BuildOneBoatForOneWoodOrOneTax(Game_phases::achat)],
                    false,
                ),
                Heroes_and_Wonders_enum::JardinSuspendus => HeroAndWonder::new(
                    "Jardin Suspendus",
                    vec![KeepTwoDifferentGood(Game_phases::achat),
                        BonusMonopole(Game_phases::revendication, Monopole::commerce, 2)],
                    false,
                ),
                Heroes_and_Wonders_enum::StatueDeZeus => HeroAndWonder::new(
                    "Statue de Zeus",
                    vec![ChoosePeaceWithAnotherPlayer(Game_phases::militaire)],
                    false,
                ),
            }
            }
    }
}