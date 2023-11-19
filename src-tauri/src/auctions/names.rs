use super::constants::REFORGES;

fn normalize_pet_name(pet_name: &mut String) {
    // Fix pet names (e.g. [Lvl 1] Dolphin)
    if pet_name.contains("[Lvl") {
        let split: Vec<&str> = pet_name.split(']').collect();
        *pet_name = split[1].to_string();
    }
}

fn remove_ascii(item_name: &mut String) {
    // Remove symbols (e.g. ⚚ and ✪✪✪✪)
    *item_name = item_name.chars().filter(|c| c.is_ascii()).collect();
}

fn remove_reforges(item_name: &mut String) {
    // Remove reforges (e.g. Fierce Shadow Assassin Helmet)

    let split: Vec<&str> = item_name.split(' ').collect();
    let first_word = split[0];

    if REFORGES.contains(&first_word) {
        *item_name = item_name.replace(first_word, "");
    }
}

pub fn normalize_name(item_name: String) -> String {
    let mut cleaned: String = item_name.clone();
    
    remove_ascii(&mut cleaned);
    normalize_pet_name(&mut cleaned);
    remove_reforges(&mut cleaned); // Has to be last so symbols dont interfere 

    cleaned = cleaned.trim().to_string(); 
    return cleaned;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_symbols() {
        let mut item_name = "⚚ Fierce Shadow Assassin Helmet ✪✪✪✪".to_string();
        remove_ascii(&mut item_name);

        assert_eq!(item_name.trim(), "Fierce Shadow Assassin Helmet")
    }

    #[test]
    fn noramlize_pet() {
        let mut item_name = "[Lvl 1] Dolphin".to_string();
        normalize_pet_name(&mut item_name);

        assert_eq!(item_name.trim(), "Dolphin")
    }

    #[test]
    fn normalize_reforges() {
        let mut item_name = "Fierce Shadow Assassin Helmet".to_string();
        remove_reforges(&mut item_name);

        assert_eq!(item_name.trim(), "Shadow Assassin Helmet")
    }
}
