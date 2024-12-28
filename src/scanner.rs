use std::sync::Arc;

use btleplug::{
    api::{
        Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter, bleuuid::uuid_from_u16,
    },
    platform::Manager,
};
use futures::StreamExt;
use tokio::sync::RwLock;

use crate::registry::{Device, Registry};

pub async fn scan(registry: Arc<RwLock<Registry>>) -> anyhow::Result<()> {
    let manager = Manager::new().await?;

    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().next().expect("no adapters found");

    let mut events = central.events().await?;

    central.start_scan(ScanFilter::default()).await?;

    while let Some(event) = events.next().await {
        if let CentralEvent::ServiceDataAdvertisement { id, service_data } = event {
            if let Some(data) = service_data.get(&uuid_from_u16(0x181c)) {
                let peripherals = central.peripherals().await.unwrap();

                let Some(peripheral) = peripherals.iter().find(|p| p.id() == id) else {
                    log::warn!("got ad from unknown peripheral");
                    continue;
                };

                let Some(properties) = peripheral.properties().await.unwrap() else {
                    log::warn!("got ad from peripheral with no properties");
                    continue;
                };

                let Some(name) = properties.local_name else {
                    log::warn!("got ad from peripheral with no name");
                    continue;
                };

                let objects = crate::bthome::decode(data.as_slice()).await;

                log::trace!("{name} {objects:?}");

                let mut registry = registry.write().await;
                let device = registry
                    .devices
                    .entry(name.clone())
                    .or_insert_with(Device::new);

                for object in objects {
                    device.update(object.name, object.value);
                }

                if let Some(rssi) = properties.rssi {
                    device.update("rssi", rssi as f32);
                }

                log::debug!("{:#?}", registry);
            }
        }
    }

    Ok(())
}
