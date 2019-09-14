use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug)]
pub struct State {
    pub containers: Vec<DesiredContainer>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: String,
    pub image: String,
    pub status: String,
    pub labels: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct DesiredContainer {
    pub image: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreated {
    pub id: String
}