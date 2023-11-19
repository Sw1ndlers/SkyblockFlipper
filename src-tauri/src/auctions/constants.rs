use std::str::FromStr;

pub const REFORGES: [&str; 87] = [
    "Gentle", "Odd", "Fast", "Fair", "Epic", "Sharp", "Heroic", "Spicy", "Legendary", "Dirty",
    "Fabled", "Suspicious", "Gilded", "Warped", "Withered", "Salty", "Treacherous", "Stiff", "Lucky", "Deadly",
    "Fine", "Grand", "Hasty", "Neat", "Neat", "Rapid", "Unreal", "Awkward", "Rich", "Precise", "Spiritual",
    "Headstrong", "Clean", "Fierce", "Heavy", "Light", "Mythic", "Pure", "Smart", "Titanic", "Wise", "Perfect",
    "Necrotic", "Ancient", "Spiked", "Renowned", "Cubic", "Warped", "Reinforced", "Loving", "Ridiculous",
    "Giant", "Submerged", "Bizarre", "Itchy", "Ominous", "Pleasant", "Pretty", "Shiny", "Simple", "Strange",
    "Vivid", "Godly", "Demonic", "Forceful", "Hurtful", "Keen", "Strong", "Superior", "Unpleasant", "Zealous",
    "Silky", "Bloody", "Shaded", "Sweet", "Fruitful", "Magnetic", "Refined", "Blessed", "Moil", "Toil", "Fleet",
    "Stellar", "Mithraic", "Auspicious", "Bustling", "Loving",
];

pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Special,
    VerySpecial,
    Supreme,
}

impl FromStr for Rarity {
    type Err = ();

    fn from_str(rarity: &str) -> Result<Self, Self::Err> {
        match rarity {
            "COMMON" => Ok(Rarity::Common),
            "UNCOMMON" => Ok(Rarity::Uncommon),
            "RARE" => Ok(Rarity::Rare),
            "EPIC" => Ok(Rarity::Epic),
            "LEGENDARY" => Ok(Rarity::Legendary),
            "MYTHIC" => Ok(Rarity::Mythic),
            "SPECIAL" => Ok(Rarity::Special),
            "VERY_SPECIAL" => Ok(Rarity::VerySpecial),
            "SUPREME" => Ok(Rarity::Supreme),
            _ => panic!("Unknown rarity {}", rarity),
        }
    }
}