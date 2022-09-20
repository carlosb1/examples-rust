use libp2p::floodsub::Topic;
use libp2p::identity;
use libp2p::PeerId;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

const STORAGE_FILE_PATH: &str = "./recipes.json";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    id: usize,
    name: String,
    ingredients: String,
    instructions: String,
    public: bool,
}

fn main() {
    println!("Hello, world!");
}
