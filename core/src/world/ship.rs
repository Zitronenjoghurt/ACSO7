mod resources;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Ship {
    pub res: resources::ShipResources,
}
