pub fn evaluate_closures_1() {
    let v = vec![1, 2, 3];
    v.iter()
        .enumerate()
        .for_each(|(index, value)| println!("index {}, value {}", index, value));
}

pub fn evaluate_closures_2() {
    let numbers_together = "140399923481800622623218009598281";
    for (index, symbol) in numbers_together.char_indices() {
        match (index % 3) {
            (0..=1) => print!("{}", symbol),
            _ => print!("{}\t", symbol),
        }
    }
}

////////////////////////////////////////////

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    pub fn new(company_name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            _ => Some(ceo.to_string()),
        };
        Self {
            name: company_name.to_string(),
            ceo,
        }
    }

    pub fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

pub fn evaluate_closures_3() {
    let v = vec![
        Company::new("company 1", "CEO 1"),
        Company::new("company 2", ""),
        Company::new("company 3", "CEO 3"),
    ];

    let v_filtered = v
        .into_iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();
    println!("{:?}", v_filtered);
}
