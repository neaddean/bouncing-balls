use specs::{Entities, System};
use tracing::info;

use caelex::{CaelexClient, CaelexClientBuilder};
use uuid::Uuid;
use tracing::debug;

pub struct CaelexSystem {
    client: CaelexClient,
}

impl CaelexSystem {
    pub fn new() -> Self {
        let mut client = CaelexClientBuilder::new()
            .max_subscriptions(100_000)
            .build()
            .unwrap();

        let entity_ids = client.get_entities(Uuid::new_v4().to_string());
        client.subscribe_entity(String::from(&entity_ids[0]));

        CaelexSystem { client }
    }
}

impl<'a> System<'a> for CaelexSystem {
    type SystemData = Entities<'a>;

    fn run(&mut self, data: Self::SystemData) {
        if let Some(updates) = self.client.get_updates() {
            debug!("{:?}", updates);
        }
    }
}
