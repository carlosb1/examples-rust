use std::default;
use std::fmt;

struct Example {
    param: String,
    param2: u32,
    param3: String,
    optional: Option<String>,
    optional2: Option<String>,
}

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "param={}, param2={}, param3={}, optional={:?}, optional2={:?}",
            self.param, self.param2, self.param3, self.optional, self.optional2
        )
    }
}
impl Default for Example {
    fn default() -> Self {
        Example {
            param: "default".to_string(),
            param2: 1,
            param3: "default3".to_string(),
            optional: None,
            optional2: Some("optional2".to_string()),
        }
    }
}

pub fn main() {
    println!(
        "example={:}",
        Example {
            param3: "anotherparam".to_string(),
            ..Default::default()
        }
    );

    println!(
        "example2={:}",
        Example {
            param2: 2,
            ..Default::default()
        }
    );
}
