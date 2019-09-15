use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone,Serialize, Deserialize,Debug)]
pub struct State {
    pub containers: Vec<DesiredContainer>
}
#[derive(Serialize, Deserialize,Debug)]
pub struct ActualState {
    pub containers: Vec<Container>
}

#[derive(Serialize,Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: String,
    pub image: String,
    pub status: String,
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DesiredContainer {
    pub image: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreatedContainer {
    pub id: String
}