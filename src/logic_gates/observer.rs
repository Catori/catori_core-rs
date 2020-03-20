use serde_derive::{Deserialize, Serialize};
use serde_lexpr::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug)]
pub struct Observer();
