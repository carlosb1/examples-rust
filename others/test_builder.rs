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

#[derive(Default)]
struct ExampleBuilder {
    param: String,
    param2: u32,
    param3: String,
    optional: Option<String>,
    optional2: Option<String>,
}

impl ExampleBuilder {
    fn new(param: String, param2: u32, param3: String) -> Self {
        ExampleBuilder {
            param,
            param2,
            param3,
            ..Default::default()
        }
    }
    pub fn optional(mut self, optional: String) -> Self {
        self.optional = Some(optional);
        return self;
    }
    pub fn optional2(mut self, optional2: String) -> Self {
        self.optional2 = Some(optional2);
        return self;
    }
    pub fn build(self) -> Example {
        Example {
            param: self.param,
            param2: self.param2,
            param3: self.param3,
            optional: self.optional,
            optional2: self.optional2,
        }
    }
}

pub fn main() {
    let example = ExampleBuilder::new("param1".to_string(), 1, "param3".to_string()).build();

    let example2 = ExampleBuilder::new("param1".to_string(), 1, "param3".to_string())
        .optional("option1".to_string())
        .optional2("option2".to_string())
        .build();

    println!("example={:}", example);
    println!("example2={:}", example2);
}
