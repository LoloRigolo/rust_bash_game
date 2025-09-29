pub const ALIASES: &[(&str, &str)] = &[
    // help
    ("h", "help"),
    ("aide", "help"),
    ("?", "help"),

    // exit
    ("quit", "exit"),
    ("q", "exit"),
    ("e", "exit"),
];

pub fn resolve(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    for (alias, canonical) in ALIASES {
        if &lower == alias {
            return (*canonical).to_string();
        }
    }
    lower
}