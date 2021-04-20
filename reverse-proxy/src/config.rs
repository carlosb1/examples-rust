use std::collections::HashMap;
use std::net::*;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

type HashAddressInfo = HashMap<String, HashMap<String, String>>;
type HashContainsYaml = HashMap<String, HashAddressInfo>;

//TODO add constructor for this case.
pub struct Host {
    pub host: String,
    pub port: i32,
}

pub struct RuleConfig {
    pub name: String,
    pub input: Host,
    pub output: Host,
}

trait Filter {
    fn is_valid(&self, rule_config: &RuleConfig) -> bool;
}

pub struct WellformedHost {}

impl Filter for WellformedHost {
    fn is_valid(&self, rule_config: &RuleConfig) -> bool {
        return !rule_config.input.host.is_empty();
    }
}

pub struct ResolvedAddress {
    resolver: Resolver,
}
impl ResolvedAddress {
    pub fn new() -> ResolvedAddress {
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
        ResolvedAddress { resolver }
    }
}

impl Filter for ResolvedAddress {
    fn is_valid(&self, rule_config: &RuleConfig) -> bool {
        self.resolver
            .lookup_ip(rule_config.input.host.as_str())
            .map_or(false, |response| response.iter().next().is_some())
    }
}

pub fn load_config(config_file: &str) -> Vec<RuleConfig> {
    info!("Loading configuration {}", config_file);
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(config_file))
        .unwrap();
    let mut doc = settings.try_into::<HashContainsYaml>().unwrap();

    let mut rules: Vec<RuleConfig> = Vec::new();
    for (name, value) in doc.iter_mut() {
        let address_input = value.clone().get_mut("input").map_or(
            Host {
                host: "".to_owned(),
                port: 0,
            },
            |v| Host {
                host: v.get("host").unwrap_or(&"".to_owned()).clone(),
                port: v
                    .get("port")
                    .unwrap_or(&"".to_owned())
                    .clone()
                    .parse::<i32>()
                    .unwrap(),
            },
        );
        let address_output = value.clone().get_mut("output").map_or(
            Host {
                host: "".to_owned(),
                port: 0,
            },
            |v| Host {
                host: v.get("host").unwrap_or(&"".to_owned()).clone(),
                port: v
                    .get("port")
                    .unwrap_or(&"".to_owned())
                    .clone()
                    .parse::<i32>()
                    .unwrap(),
            },
        );

        rules.push(RuleConfig {
            name: name.to_string(),
            input: address_input,
            output: address_output,
        });
    }

    return rules;
}
