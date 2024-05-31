use ressources::RessourceType;
struct Player {
    heroes_and_wonder: Vect<Heroes_and_Wonders_enum>,
    ressources: Vect<(RessourceType,u8)>,
    territoires: Vect<&Territoire>,
}