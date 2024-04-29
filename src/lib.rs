pub struct Roman();

const ROMAN: [(usize, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Roman {
    pub fn from(mut num: usize) -> String {
        let mut added_symbol = String::new();

        for (value, symbol) in ROMAN {
             while num >= value {
                 added_symbol.push_str(symbol);
                 num -= value;
             }
        }

        added_symbol
    }
}

