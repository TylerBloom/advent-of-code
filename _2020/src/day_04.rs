pub use crate::solution::Solution;

use regex::Regex;

use std::collections::HashMap;

fn generate_parser() -> HashMap<String, Regex> {
    let mut digest: HashMap<String, Regex> = HashMap::with_capacity(7);
    digest.insert(
        String::from("byr"),
        Regex::new(r"byr:([0-9]{4})( |\n|$)").unwrap(),
    );
    digest.insert(
        String::from("iyr"),
        Regex::new(r"iyr:([0-9]{4})( |\n|$)").unwrap(),
    );
    digest.insert(
        String::from("eyr"),
        Regex::new(r"eyr:([0-9]{4})( |\n|$)").unwrap(),
    );
    digest.insert(
        String::from("hgt"),
        Regex::new(r"hgt:([0-9]+)(in|cm)( |\n|$)").unwrap(),
    );
    digest.insert(
        String::from("hcl"),
        Regex::new(r"hcl:#[0-9a-f]{6}( |\n|$)").unwrap(),
    );
    digest.insert(
        String::from("ecl"),
        Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)( |\n|$)").unwrap(),
    );
    digest.insert(
        String::from("pid"),
        Regex::new(r"pid:[0-9]{9}( |\n|$)").unwrap(),
    );
    //digest.insert(String::from("cid"), Regex::new(r"(|pid:[0-9]+)").unwrap());
    digest
}

struct Passport {
    data: String,
}

impl Passport {
    fn is_valid_loose(&self) -> bool {
        let mut digest = true;
        digest &= self.data.contains("byr");
        digest &= self.data.contains("iyr");
        digest &= self.data.contains("eyr");
        digest &= self.data.contains("hgt");
        digest &= self.data.contains("hcl");
        digest &= self.data.contains("ecl");
        digest &= self.data.contains("pid");
        //digest &= self.data.contains("cid");
        digest
    }

    fn is_valid_strict(&self, rules: &HashMap<String, Regex>) -> bool {
        let mut digest = true;
        for (s, r) in rules.into_iter() {
            if let Some(caps) = r.captures(&self.data) {
                digest &= caps.len() > 0;
                match s.as_str() {
                    "byr" => {
                        let tmp = caps[1].to_string().parse::<u32>().unwrap();
                        digest &= (tmp >= 1920) && (tmp <= 2002);
                    }
                    "iyr" => {
                        let tmp = caps[1].to_string().parse::<u32>().unwrap();
                        digest &= (tmp >= 2010) && (tmp <= 2020);
                    }
                    "eyr" => {
                        let tmp = caps[1].to_string().parse::<u32>().unwrap();
                        digest &= (tmp >= 2020) && (tmp <= 2030);
                    }
                    "hgt" => {
                        let tmp = caps[1].to_string().parse::<u32>().unwrap();
                        if &caps[2] == "cm" {
                            digest &= (tmp >= 150) && (tmp <= 193);
                        } else {
                            digest &= (tmp >= 59) && (tmp <= 76);
                        }
                    }
                    _ => {}
                }
            } else {
                digest = false;
            }
            if !digest {
                break;
            }
        }
        digest
    }
}

// Storing the "strict" rules for part 2 isn't ideal, but compiling the regex
// for every Passport (or worth every is_valid_strict call) is awful for
// performance. Alternatively, these could be static global values....
pub struct Day4 {
    data: Vec<Passport>,
    parser: HashMap<String, Regex>,
}

impl Solution<u64> for Day4 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n\n").collect();
        let mut data: Vec<Passport> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(Passport {
                    data: String::from(val),
                });
            }
        }
        Day4 {
            data,
            parser: generate_parser(),
        }
    }

    fn solve_part_one(&self) -> u64 {
        self.data.iter().filter(|p| p.is_valid_loose()).count() as u64
    }

    fn solve_part_two(&self) -> u64 {
        self.data
            .iter()
            .filter(|p| p.is_valid_strict(&self.parser))
            .count() as u64
    }
}

#[cfg(test)]
mod tests {

    use super::{Day4, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in");
        let solver = Day4::parse_input(input);
        assert_eq!(solver.solve_part_one(), 2);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007");
        let solver = Day4::parse_input(input);
        assert_eq!(solver.solve_part_two(), 0);
    }

    #[test]
    fn test_part_three() {
        let input = String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");
        let solver = Day4::parse_input(input);
        assert_eq!(solver.solve_part_two(), 4);
    }

    #[test]
    fn test_part_four() {
        let input = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in");
        let solver = Day4::parse_input(input);
        assert_eq!(solver.solve_part_one(), 2);
    }
}
