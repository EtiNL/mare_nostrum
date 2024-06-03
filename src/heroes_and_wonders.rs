
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
        BonusMonopole(GamePhases, Monopole),
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
        pub fn get_ability(&self) -> Vec<Ability> {
            match self {
                HeroesAndWondersEnum::Cesar => vec![Ability::AttackDice(GamePhases::Militaire, Terrain::Terre, 1)],

                HeroesAndWondersEnum::Cleopatre => vec![Ability::IncludeTaxTokenInGoodCombination(GamePhases::Achats), Ability::IncludeGoodTokenInTaxCombination(GamePhases::Achats)],

                HeroesAndWondersEnum::Hammourabi => vec![Ability::FreeInfluenceMarker(GamePhases::Achats)],

                HeroesAndWondersEnum::Hannibal => vec![Ability::SameGoodTokenInGoodCombination(GamePhases::Achats)],

                HeroesAndWondersEnum::Pericles => vec![Ability::DefenceDice(GamePhases::Militaire, Terrain::Terre, 2)],

                HeroesAndWondersEnum::Antigone => vec![Ability::ExengeWithTheNumberOfGoodPlayerWants(GamePhases::Commerce),
                                                        Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial(2)),
                                                        Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique(2))],

                HeroesAndWondersEnum::CastorEtPolux => vec![Ability::CopyAbility],

                HeroesAndWondersEnum::Circe => vec![Ability::UseLegionAsCaravane(GamePhases::Militaire),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial(1)),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire(1))],

                HeroesAndWondersEnum::Gilgamesh => vec![Ability::ContructForteresseForPierreOrImpot(GamePhases::Achats)],

                HeroesAndWondersEnum::Hamilcar => vec![Ability::DoubleBeneficePillage(GamePhases::Militaire),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial(1)),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique(1)),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire(1))],

                HeroesAndWondersEnum::Hercule => vec![Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial(2)),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique(2)),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire(2))],

                HeroesAndWondersEnum::Nabuchodonosor => vec![Ability::Plus5InBattleIfOwnTerritory(GamePhases::Militaire)],

                HeroesAndWondersEnum::Pentesilee => vec![Ability::FreeLegionInOccupiedTerritory(GamePhases::Achats),
                                                        Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique(2)),
                                                        Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire(2))],

                HeroesAndWondersEnum::Persee => vec![Ability::PlayFirstForTheWin(GamePhases::Achats),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial(1)),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique(2)),
                                                    Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire(1))],

                HeroesAndWondersEnum::RamsesII => vec![Ability::OneMoreDice(GamePhases::Militaire)],

                HeroesAndWondersEnum::ReineDeSaba => vec![Ability::AutomaticallyConvertTerritoryIfLeggionOnInfluenceMarker(GamePhases::Militaire),
                                                        Ability::BonusMonopole(GamePhases::Revendication, Monopole::Politique(1)),
                                                        Ability::BonusMonopole(GamePhases::Revendication, Monopole::Militaire(1))],

                HeroesAndWondersEnum::Spartacus => vec![Ability::LegionForGladiatorOrOneTax(GamePhases::Achats)],

                HeroesAndWondersEnum::Colosse => vec![Ability::TakeOneGoodThatPlayerWantsAfterRessourceCollection(GamePhases::Production)],

                HeroesAndWondersEnum::TempleArtemis => vec![Ability::OneTaxFree(GamePhases::Production)],

                HeroesAndWondersEnum::Phare => vec![Ability::BuildOneBoatForOneWoodOrOneTax(GamePhases::Achats)],

                HeroesAndWondersEnum::JardinSuspendus => vec![Ability::KeepTwoDifferentGood(GamePhases::Achats),
                                                            Ability::BonusMonopole(GamePhases::Revendication, Monopole::Commercial(2))],

                HeroesAndWondersEnum::StatueDeZeus => vec![Ability::ChoosePeaceWithAnotherPlayer(GamePhases::Militaire)],
            }
            }
    }
}