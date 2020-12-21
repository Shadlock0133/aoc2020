use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/day21.txt").unwrap();
    let foods = parse_input(&input);
    let res = check_1(&foods);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&foods);
    println!("Part 2 - Answer: {}", res);
}

#[derive(Debug)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

fn parse_input(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let (ings, alls) = line.split_at(line.find(" (").unwrap());
            let ingredients = ings.split_whitespace().collect();
            let allergens = alls
                .trim()
                .strip_prefix("(contains ")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(", ")
                .collect();
            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn map_allergens<'a>(foods: &[Food<'a>]) -> HashMap<&'a str, &'a str> {
    let mut possible_allergens = HashMap::<&str, HashSet<&str>>::new();
    for food in foods {
        for allergen in &food.allergens {
            possible_allergens
                .entry(allergen)
                .or_default()
                .extend(&food.ingredients);
        }
    }
    for (allergen, names) in &mut possible_allergens {
        let iter = foods
            .iter()
            .filter(|x| x.allergens.contains(allergen))
            .map(|x| x.ingredients.iter().copied().collect());
        for i in iter {
            *names = names.intersection(&i).copied().collect();
        }
    }
    let mut allergens = HashMap::new();
    while !possible_allergens.is_empty() {
        let (solved, rest) = possible_allergens.into_iter().partition(|x| x.1.len() == 1);
        possible_allergens = rest;
        allergens.extend(
            solved
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().next().unwrap())),
        );
        for names in possible_allergens.values_mut() {
            for i in allergens.values() {
                names.remove(i);
            }
        }
    }
    assert!(possible_allergens.is_empty());
    allergens
}

fn check_1(foods: &[Food]) -> usize {
    let allergen_names = map_allergens(foods)
        .values()
        .copied()
        .collect::<Vec<&str>>();
    foods
        .iter()
        .flat_map(|x| x.ingredients.iter())
        .filter(|x| !allergen_names.contains(x))
        .count()
}

fn check_2(foods: &[Food]) -> String {
    let mut allergen_names = map_allergens(foods).into_iter().collect::<Vec<_>>();
    allergen_names.sort_unstable_by_key(|x| x.0);
    allergen_names.into_iter().map(|x| x.1).collect::<Vec<_>>().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test1() {
        let output = 5;
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = "mxmxvkd,sqjhc,fvjkl";
        let res = check_2(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}
