
use crate::monopole_counter::monopole_counter::Monopole;
use crate::game_phases::game_phases::GamePhases;
use crate::plateau::plateau::Terrain;

pub mod heroes_and_wonders_mod {
    use super::{Monopole, GamePhases, Terrain};
    
    #[derive(Debug, Clone)]
    pub enum Ability {
        AttackDice(GamePhases, Terrain, u8),
        IncludeTaxTokenInGoodCombination(GamePhases),
        IncludeGoodTokenInTaxCombination(GamePhases),
        FreeInfluenceMarker(GamePhases),
        SameGoodTokenInGoodCombination(GamePhases),
        DefenceDice(GamePhases, Terrain, u8),
        ExengeWithTheNumberOfGoodPlayerWants(GamePhases),
        BonusMonopole(GamePhases, Monopole, u8),
        CopyAbility,
        UseLegionAsCaravane(GamePhases),
        ContructForteresseForPierreOrImpot(GamePhases),
        DoubleBeneficePillage(GamePhases),
        Plus5InBattleIfOwnTerritory(GamePhases), // if there is a legion
        FreeLegionInOccupiedTerritory(GamePhases),
        PlayFirstForTheWin(GamePhases),
        OneMoreDice(GamePhases),
        AutomaticallyConvertTerritoryIfLeggionOnInfluenceMarker(GamePhases),
        LegionForGladiatorOrOneTax(GamePhases),
        TakeOneGoodThatPlayerWantsAfterRessourceCollection(GamePhases),
        BuildOneBoatForOneWoodOrOneTax(GamePhases),
        OneTaxFree(GamePhases),
        KeepTwoDifferentGood(GamePhases),
        ChoosePeaceWithAnotherPlayer(GamePhases),
    }

    #[derive(Debug, Clone)]
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

    #[derive(Debug, Clone)]
    pub enum HeroesAndWondersEnum {
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

    impl HeroesAndWondersEnum {
        pub fn get_hero_and_wonder(&self) -> HeroAndWonder {
            match self {
                HeroesAndWondersEnum::Cesar => HeroAndWonder::new(
                    "Cesar",
                    vec![Ability::AttackDice(GamePhases::Militaire, Terrain::Terre, 1)],
                    true,
                ),
                HeroesAndWondersEnum::Cleopatre => HeroAndWonder::new(
                    "Cleopatre",
                    vec![Ability::IncludeTaxTokenInGoodCombination(GamePhases::Achats), Ability::IncludeGoodTokenInTaxCombination(GamePhases::Achats)],
                    true,
                ),
                HeroesAndWondersEnum::Hammourabi => HeroAndWonder::new(
                    "Hammourabi",
                    vec![Ability::FreeInfluenceMarker(GamePhases::Achats)],
                    true,
                ),
                HeroesAndWondersEnum::Hannibal => HeroAndWonder::new(
                    "Hannibal",
                    vec![Ability::SameGoodTokenInGoodCombination(GamePhases::Achats)],
                    true,
                ),
                HeroesAndWondersEnum::Pericles => HeroAndWonder::new(
                    "Pericles",
                    vec![Ability::DefenceDice(GamePhases::Militaire, Terrain::Terre, 2)],
                    true,
                ),
                HeroesAndWondersEnum::Antigone => HeroAndWonder::new(
                    "Antigone",
                    vec![Ability::ExengeWithTheNumberOfGoodPlayerWants(GamePhases::Commerce),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial, 2),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique, 2)],
                    false,
                ),
                HeroesAndWondersEnum::CastorEtPolux => HeroAndWonder::new(
                    "Castor Et Polux",
                    vec![Ability::CopyAbility],
                    false,
                ),
                HeroesAndWondersEnum::Circe => HeroAndWonder::new(
                    "Circé",
                    vec![Ability::UseLegionAsCaravane(GamePhases::Militaire),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial, 1),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire, 1)],
                    false,
                ),
                HeroesAndWondersEnum::Gilgamesh => HeroAndWonder::new(
                    "Gilgamesh",
                    vec![Ability::ContructForteresseForPierreOrImpot(GamePhases::Achats)],
                    false,
                ),
                HeroesAndWondersEnum::Hamilcar => HeroAndWonder::new(
                    "Hamilcar",
                    vec![Ability::DoubleBeneficePillage(GamePhases::Militaire),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial, 1),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique, 1),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire, 1)],
                    false,
                ),
                HeroesAndWondersEnum::Hercule => HeroAndWonder::new(
                    "Hercule",
                    vec![Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial, 2),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique, 2),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire, 2)],
                    false,
                ),
                HeroesAndWondersEnum::Nabuchodonosor => HeroAndWonder::new(
                    "Nabuchodonosor",
                    vec![Ability::Plus5InBattleIfOwnTerritory(GamePhases::Militaire)],
                    false,
                ),
                HeroesAndWondersEnum::Pentesilee => HeroAndWonder::new(
                    "Pentesilée",
                    vec![Ability::FreeLegionInOccupiedTerritory(GamePhases::Achats),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique, 2),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire, 2)],
                    false,
                ),
                HeroesAndWondersEnum::Persee => HeroAndWonder::new(
                    "Persée",
                    vec![Ability::PlayFirstForTheWin(GamePhases::Achats),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial, 1),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique, 2),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire, 1)],
                    false,
                ),
                HeroesAndWondersEnum::RamsesII => HeroAndWonder::new(
                    "Ramses II",
                    vec![Ability::OneMoreDice(GamePhases::Militaire)],
                    false,
                ),
                HeroesAndWondersEnum::ReineDeSaba => HeroAndWonder::new(
                    "Reine de Saba",
                    vec![Ability::AutomaticallyConvertTerritoryIfLeggionOnInfluenceMarker(GamePhases::Militaire),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique, 1),
                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire, 1)],
                    false,
                ),
                HeroesAndWondersEnum::Spartacus => HeroAndWonder::new(
                    "Spartacus",
                    vec![Ability::LegionForGladiatorOrOneTax(GamePhases::Achats)],
                    false,
                ),
                HeroesAndWondersEnum::Colosse => HeroAndWonder::new(
                    "Colosse",
                    vec![Ability::TakeOneGoodThatPlayerWantsAfterRessourceCollection(GamePhases::Production)],
                    false,
                ),
                HeroesAndWondersEnum::TempleArtemis => HeroAndWonder::new(
                    "Temple d'Artémis",
                    vec![Ability::OneTaxFree(GamePhases::Production)],
                    false,
                ),
                HeroesAndWondersEnum::Phare => HeroAndWonder::new(
                    "Phare",
                    vec![Ability::BuildOneBoatForOneWoodOrOneTax(GamePhases::Achats)],
                    false,
                ),
                HeroesAndWondersEnum::JardinSuspendus => HeroAndWonder::new(
                    "Jardin Suspendus",
                    vec![Ability::KeepTwoDifferentGood(GamePhases::Achats),
                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial, 2)],
                    false,
                ),
                HeroesAndWondersEnum::StatueDeZeus => HeroAndWonder::new(
                    "Statue de Zeus",
                    vec![Ability::ChoosePeaceWithAnotherPlayer(GamePhases::Militaire)],
                    false,
                ),
            }
            }
    }
}