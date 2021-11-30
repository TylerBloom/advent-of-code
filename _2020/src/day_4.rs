
pub use crate::solution::Solution;

struct Passport {
    data : String,
}

impl Passport {
    fn is_valid(&self) -> bool {
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
}

pub struct Day4 {
    data: Vec<Passport>,
}

impl Solution<u64> for Day4 {
    fn parse_input( input: String ) -> Self {
        let parsed: Vec<&str> = input.split("\n\n").collect();
        let mut data: Vec<Passport> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push( Passport{ data: String::from(val) } );
            }
        }
        Day4 { data }
    }
    
    fn solve_part_one( &self ) -> u64 {
        self.data.iter().filter( |p| p.is_valid() ).count() as u64
    }
    
    fn solve_part_two( &self ) -> u64 {
        self.data.iter().filter( |p| p.is_valid() ).count() as u64
    }
}

#[cfg(test)]
mod tests {
    
    use super::{Day4, Solution};
    
    #[test]
    fn test_part_one() {
        let input = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in");
        let solver = Day4::parse_input( input );
        assert_eq!( solver.solve_part_one(), 2 );
    }
    
    #[test]
    fn test_part_two() {
        let input = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in");
        let solver = Day4::parse_input( input );
        assert_eq!( solver.solve_part_one(), 2 );
    }
}
