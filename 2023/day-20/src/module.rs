use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Module {
    pub ctg: char,
    pub name: String,
    pub dest: Vec<String>,
}

impl Module {
    pub fn new(ctg: char, name: &str, dest: Vec<&str>) -> Self {
        return Module {
            ctg,
            name: String::from(name),
            dest: dest.iter().map(|str| String::from(*str)).collect(),
        };
    }

    pub fn from(input: &str) -> Self {
        let (left, right) = input.split_once(" -> ").unwrap();
        let right = right.split(", ").collect::<Vec<&str>>();
        return Module::new(
            left.chars().nth(0).unwrap(),
            if left == "broadcaster" {
                &left
            } else {
                &left[1..]
            },
            right,
        );
    }

    pub fn flipflop(&self) -> Option<(&Self, bool)> {
        if self.ctg == '%' {
            Some((self, false))
        } else {
            None
        }
    }

    pub fn conjunction(&self) -> Option<(&Self, HashMap<String, bool>)> {
        if self.ctg == '&' {
            Some((self, HashMap::new()))
        } else {
            None
        }
    }
}
