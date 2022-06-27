use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Encounter {
    Grass,
    DoubleGrass,
    SpecialGrass,
    RockSmash,
    Surfing,
    SpecialSurf,
    OldRod,
    GoodRod,
    SuperRod,
    SpecialSuperRod,
    Static,
    BugCatchingContest,
    Headbutt,
    Roamer,
    Gift,
    EntraLink,
    GiftEgg,
    HiddenGrotto,
}

impl Display for Encounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Encounter::Grass => write!(f, "Grass"),
            Encounter::DoubleGrass => write!(f, "Double Grass"),
            Encounter::SpecialGrass => write!(f, "Special Grass"),
            Encounter::RockSmash => write!(f, "Rock Smash"),
            Encounter::Surfing => write!(f, "Surfing"),
            Encounter::SpecialSurf => write!(f, "Special Surf"),
            Encounter::OldRod => write!(f, "Old Rod"),
            Encounter::GoodRod => write!(f, "Good Rod"),
            Encounter::SuperRod => write!(f, "Super Rod"),
            Encounter::SpecialSuperRod => write!(f, "Special Super Rod"),
            Encounter::Static => write!(f, "Static"),
            Encounter::BugCatchingContest => write!(f, "Bug Catching Contest"),
            Encounter::Headbutt => write!(f, "Headbutt"),
            Encounter::Roamer => write!(f, "Roamer"),
            Encounter::Gift => write!(f, "Gift"),
            Encounter::EntraLink => write!(f, "Entra Link"),
            Encounter::GiftEgg => write!(f, "Gift Egg"),
            Encounter::HiddenGrotto => write!(f, "Hidden Grotto"),
        }
    }
}
