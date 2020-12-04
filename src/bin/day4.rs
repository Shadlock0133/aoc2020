use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    let passports = parse_input(&input).unwrap();
    let res = passports.iter().filter(|x| check_1(x)).count();
    println!("Part 1 - Answer: {}", res);
    let res = passports.iter().filter(|x| check_1(x) && check_2(x)).count();
    println!("Part 2 - Answer: {}", res);
}

type Passport<'a> = HashMap<&'a str, &'a str>;

fn parse_input(input: &str) -> Result<Vec<Passport>, Box<dyn std::error::Error>> {
    let mut res = vec![];
    let mut current_pass = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            res.push(std::mem::take(&mut current_pass));
        } else {
            let fields = line
                .split_whitespace()
                .map(|x| {
                    let mut split = x.split(':');
                    let name = split.next().unwrap();
                    let value = split.next().unwrap();
                    (name, value)
                });
            current_pass.extend(fields);
        }
    }
    if !current_pass.is_empty() {
        res.push(current_pass);
    }
    Ok(res)
}

fn check_1(passport: &Passport) -> bool {
    passport.contains_key("byr")
    && passport.contains_key("iyr")
    && passport.contains_key("eyr")
    && passport.contains_key("hgt")
    && passport.contains_key("hcl")
    && passport.contains_key("ecl")
    && passport.contains_key("pid")
}

fn check_range(value: &str, min: u16, max: u16) -> bool {
    let value = value.parse().unwrap();
    (min..=max).contains(&value)
}

const EYE_COLOURS: &[&str] = &["amb","blu", "brn", "gry", "grn", "hzl", "oth"];

fn check_2(passport: &Passport) -> bool {
    for (&name, &value) in passport {
        match name {
            "byr" if !check_range(value, 1920, 2002) => return false,
            "iyr" if !check_range(value, 2010, 2020) => return false,
            "eyr" if !check_range(value, 2020, 2030) => return false,
            "hgt" =>
                if let Some(value) = value.strip_suffix("cm") {
                    if !check_range(value, 150, 193) {
                        return false;
                    }
                } else if let Some(value) = value.strip_suffix("in") {
                    if !check_range(value, 59, 76) {
                        return false;
                    }
                } else {
                    return false;
                }
            "hcl" =>
                if let Some(value) = value.strip_prefix('#') {
                    if value.chars().count() != 6 || !value.chars().all(|x| x.is_ascii_hexdigit()) {
                        return false;
                    }
                } else {
                    return false;
                }
            "ecl" if !EYE_COLOURS.contains(&value) => return false,
            "pid" =>
                if value.chars().count() != 9 || !value.chars().all(|x| x.is_ascii_digit()) {
                    return false;
                }
            _ => (),
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test1() {
        let output = 2;
        let passports = parse_input(INPUT).unwrap();
        let res = passports.into_iter().filter(|x| check_1(x)).count();
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        const INVALID: &str =
            "eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
            
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
            
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007";
        const VALID: &str =
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f
            
            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
            
            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022
            
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        
        let invalid = parse_input(INVALID).unwrap().iter().all(|x| !check_2(x));
        assert!(invalid);
        let valid = parse_input(VALID).unwrap().iter().all(|x| check_2(x));
        assert!(valid);
    }
}
