
/// Ensures that the name provided is unique relative to the list of other names.
/// If it is not, then it will create a new unique name.
pub fn ensure_unique_name(name: &str, relative_to: Vec<String>) -> String {
    
    // Strip the name bare of any numerical suffix.
    let given_value:         Option<usize> = extract_numerical_suffix(name);
    let name_without_suffix: String        = match given_value {
        Some(number) => name.split_at(name.find(&format!("{}", number)).unwrap()).0.to_string(),
        None         => String::new()
    };
    
    // Search for any similar names that have the same beginning but different suffixes.
    let mut similar_names: Vec<String> = Vec::new();
    for set_name in &relative_to {
        let idx_found: Option<usize> = set_name.find(&name_without_suffix);
        
        if let Some(idx) = idx_found {
            if idx != 0 {   // We do not include similar names when the pattern does not start at the beginning of the string.
                continue;
            }
            similar_names.push(set_name.to_string());
        }
    }

    if similar_names.len() == 0 {
        return name.to_string();
    }

    // Order all of the names with a numerical suffix.
    // If this name does not have a numerical suffix, then give it the lowest possible numerical
    // suffix.
    // Otherwise, give it the closest numerical suffix to the one it currently has (counting
    // upwards).
    fn extract_numerical_suffix(s: &str) -> Option<usize> {
        let mut numerics: String = String::new();
        let mut ptr:      usize  = s.len() - 1;
        
        loop {
            let char: char = s.get(ptr..(ptr + 1)).unwrap().chars().collect::<Vec<_>>()[0];
            if !char.is_numeric() {
                break;
            }
            numerics = char.to_string() + &numerics;
            
            if ptr == 0 {
                break;
            }
            ptr -= 1;
        }

        if numerics.is_empty() {
            return None;
        }
        Some(numerics.parse::<usize>().unwrap())
    }
    
    let mut new_value:          usize  = given_value.unwrap_or(0);
    let     values:         Vec<usize> = similar_names.iter().map(|n| extract_numerical_suffix(n).unwrap_or(0)).collect(); // If there are no numerical suffixes on similar names,
                                                                                                                           // then we give them a baseline value.
    loop {
        for value in values {
            if new_value == value {
                new_value += 1;
                continue;
            }
        }
        break;
    }
    
    let new_suffix: String = format!("{}", new_value);
    name_without_suffix.to_string() + &new_suffix
}
