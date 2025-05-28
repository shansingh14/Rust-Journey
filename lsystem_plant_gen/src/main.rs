use std::collections::HashMap;
use turtle::Turtle;

struct Rule {
    from: char,
    to: String,
}

fn create_rule_map(rules: &[Rule]) -> HashMap<char, String> {
    rules
        .iter()
        .map(|r: &Rule| (r.from, r.to.clone()))
        .collect()
}

fn main() {
    let rules: [Rule; 2] = [
        Rule {
            from: '1',
            to: String::from("11").into(),
        },
        Rule {
            from: '0',
            to: String::from("1[0]0").into(),
        },
    ];

    let mut axiom = String::from("A");
    gen_sequence(&axiom, &rules, 3);
}

fn gen_sequence(axiom: &str, rules: &[Rule], n: usize) {
    let rule_map = create_rule_map(rules);
    let mut sequence = axiom.to_string();

    for i in 0..n {
        sequence = expand_once(&sequence, &rule_map);
        println!("Gen {} : {}", i, sequence)
    }
}

fn expand_once(axiom: &str, rule_map: &HashMap<char, String>) -> String {
    let mut stem: String = String::new();
    for chr in axiom.to_string().chars() {
        let mut rep = rule_map.get(&chr).unwrap();
        if rep != "" {
            stem.push_str(rep);
        } else {
            stem.push(chr);
        }
    }

    return stem;
}
