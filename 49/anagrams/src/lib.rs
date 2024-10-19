use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut acc: HashMap<String, Vec<String>> = HashMap::new();
    strs.into_iter().for_each(|s| {
        let mut tmp: Vec<char> = s.clone().chars().collect();
        tmp.sort();
        let tmp: String = tmp.iter().collect();
        acc.entry(tmp).or_insert(Vec::new()).push(s)
    });
    acc.values().cloned().collect()
}

#[cfg(test)]
mod tests {}
