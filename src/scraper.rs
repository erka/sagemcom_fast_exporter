use crate::client::Client;
use crate::error::Result;

#[allow(non_snake_case)]
#[allow(dead_code)]
mod api {
    use serde::Deserialize;
    use serde::de;

    fn deserialize_string_or_num<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::String(s) => s.parse().map_err(de::Error::custom),
            serde_json::Value::Number(n) => n.as_u64().ok_or_else(|| de::Error::custom("invalid number")),
            _ => Err(de::Error::custom("expected string or number")),
        }
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct DeviceResponse {
        #[serde(rename = "Device")]
        pub device: Device,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct Device {
        #[serde(rename = "DeviceInfo")]
        #[serde(default)]
        pub device_info: DeviceInfo,
        #[serde(rename = "WiFi")]
        #[serde(default)]
        pub wifi: Wifi,
        #[serde(rename = "Ethernet")]
        #[serde(default)]
        pub ethernet: Ethernet,
        #[serde(rename = "Optical")]
        #[serde(default)]
        pub optical: Optical,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    #[serde(from = "DeviceInfoRaw")]
    pub struct DeviceInfo {
        pub up_time: u64,
        pub reboot_count: u64,
        pub memory_status: MemoryStatus,
        pub product_class: String,
        pub serial_number: String,
        pub model_name: String,
        pub manufacturer: String,
        pub software_version: String,
        pub hardware_version: String,
        pub mac_address: String,
        pub country: String,
        pub description: String,
        pub additional_hardware_version: String,
        pub additional_software_version: String,
        pub backup_software_version: String,
        pub external_firmware_version: String,
        pub gui_api_version: String,
        pub gui_firmware_version: String,
        pub internal_firmware_version: String,
        pub manufacturer_oui: String,
        pub mode: String,
        pub model_number: String,
        pub ont_serial_number: String,
        pub provisioning_code: String,
        pub router_name: String,
        pub spec_version: String,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    struct DeviceInfoRaw {
        #[serde(rename = "UpTime", default)]
        up_time: Option<u64>,
        #[serde(rename = "RebootCount", default)]
        reboot_count: Option<u64>,
        #[serde(rename = "MemoryStatus", default)]
        memory_status: Option<MemoryStatus>,
        #[serde(rename = "ProductClass")]
        #[serde(default)]
        product_class: String,
        #[serde(rename = "SerialNumber")]
        #[serde(default)]
        serial_number: String,
        #[serde(rename = "ModelName")]
        #[serde(default)]
        model_name: String,
        #[serde(rename = "Manufacturer")]
        #[serde(default)]
        manufacturer: String,
        #[serde(rename = "SoftwareVersion")]
        #[serde(default)]
        software_version: String,
        #[serde(rename = "HardwareVersion")]
        #[serde(default)]
        hardware_version: String,
        #[serde(rename = "MACAddress")]
        #[serde(default)]
        mac_address: String,
        #[serde(rename = "Country")]
        #[serde(default)]
        country: String,
        #[serde(rename = "Description")]
        #[serde(default)]
        description: String,
        #[serde(rename = "AdditionalHardwareVersion")]
        #[serde(default)]
        additional_hardware_version: String,
        #[serde(rename = "AdditionalSoftwareVersion")]
        #[serde(default)]
        additional_software_version: String,
        #[serde(rename = "BackupSoftwareVersion")]
        #[serde(default)]
        backup_software_version: String,
        #[serde(rename = "ExternalFirmwareVersion")]
        #[serde(default)]
        external_firmware_version: String,
        #[serde(rename = "GUIAPIVersion")]
        #[serde(default)]
        gui_api_version: String,
        #[serde(rename = "GUIFirmwareVersion")]
        #[serde(default)]
        gui_firmware_version: String,
        #[serde(rename = "InternalFirmwareVersion")]
        #[serde(default)]
        internal_firmware_version: String,
        #[serde(rename = "ManufacturerOUI")]
        #[serde(default)]
        manufacturer_oui: String,
        #[serde(rename = "Mode")]
        #[serde(default)]
        mode: String,
        #[serde(rename = "ModelNumber")]
        #[serde(default)]
        model_number: String,
        #[serde(rename = "ONTSerialNumber")]
        #[serde(default)]
        ont_serial_number: String,
        #[serde(rename = "ProvisioningCode")]
        #[serde(default)]
        provisioning_code: String,
        #[serde(rename = "RouterName")]
        #[serde(default)]
        router_name: String,
        #[serde(rename = "SpecVersion")]
        #[serde(default)]
        spec_version: String,
    }

    impl From<DeviceInfoRaw> for DeviceInfo {
        fn from(raw: DeviceInfoRaw) -> Self {
            DeviceInfo {
                up_time: raw.up_time.unwrap_or(0),
                reboot_count: raw.reboot_count.unwrap_or(0),
                memory_status: raw.memory_status.unwrap_or_default(),
                product_class: raw.product_class,
                serial_number: raw.serial_number,
                model_name: raw.model_name,
                manufacturer: raw.manufacturer,
                software_version: raw.software_version,
                hardware_version: raw.hardware_version,
                mac_address: raw.mac_address,
                country: raw.country,
                description: raw.description,
                additional_hardware_version: raw.additional_hardware_version,
                additional_software_version: raw.additional_software_version,
                backup_software_version: raw.backup_software_version,
                external_firmware_version: raw.external_firmware_version,
                gui_api_version: raw.gui_api_version,
                gui_firmware_version: raw.gui_firmware_version,
                internal_firmware_version: raw.internal_firmware_version,
                manufacturer_oui: raw.manufacturer_oui,
                mode: raw.mode,
                model_number: raw.model_number,
                ont_serial_number: raw.ont_serial_number,
                provisioning_code: raw.provisioning_code,
                router_name: raw.router_name,
                spec_version: raw.spec_version,
            }
        }
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct MemoryStatus {
        #[serde(rename = "Total", default)]
        pub total: i64,
        #[serde(rename = "Free", default)]
        pub free: i64,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct Wifi {
        #[serde(rename = "Radios")]
        #[serde(default)]
        pub radios: Vec<Radio>,
        #[serde(rename = "SSIDs")]
        #[serde(default)]
        pub ssids: Vec<Ssid>,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct Radio {
        #[serde(rename = "Name")]
        #[serde(default)]
        pub name: String,
        #[serde(rename = "Alias")]
        #[serde(default)]
        pub alias: String,
        #[serde(rename = "Status")]
        #[serde(default)]
        pub status: String,
        #[serde(rename = "Channel")]
        #[serde(default)]
        pub channel: u64,
        #[serde(rename = "MaxBitRate")]
        #[serde(default)]
        pub max_bit_rate: u64,
        #[serde(rename = "CurrentOperatingChannelBandwidth")]
        #[serde(default)]
        pub current_operating_channel_bandwidth: String,
        #[serde(rename = "TransmitPower")]
        #[serde(default)]
        pub transmit_power: f64,
        #[serde(rename = "TransmitPowerMax")]
        #[serde(default)]
        pub transmit_power_max: f64,
        #[serde(rename = "RegulatoryDomain")]
        #[serde(default)]
        pub regulatory_domain: String,
        #[serde(rename = "SupportedStandards")]
        #[serde(default)]
        pub supported_standards: String,
        #[serde(rename = "SupportedChannelBandwidth")]
        #[serde(default)]
        pub supported_channel_bandwidth: String,
        #[serde(rename = "LastChange")]
        #[serde(default)]
        pub last_change: u64,
        #[serde(rename = "IfcName")]
        #[serde(default)]
        pub ifc_name: String,
        #[serde(rename = "Stats")]
        #[serde(default)]
        pub stats: RadioStats,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct RadioStats {
        #[serde(rename = "Noise")]
        #[serde(default)]
        pub noise: f64,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct Ssid {
        #[serde(rename = "Name")]
        #[serde(default)]
        pub name: String,
        #[serde(rename = "Alias")]
        #[serde(default)]
        pub alias: String,
        #[serde(rename = "Status")]
        #[serde(default)]
        pub status: String,
        #[serde(rename = "SSID")]
        #[serde(default)]
        pub ssid: String,
        #[serde(rename = "Enable")]
        #[serde(default)]
        pub enable: bool,
        #[serde(rename = "IfcName")]
        #[serde(default)]
        pub ifc_name: String,
        #[serde(rename = "LowerLayers")]
        #[serde(default)]
        pub lower_layers: String,
        #[serde(rename = "MACAddress")]
        #[serde(default)]
        pub mac_address: String,
        #[serde(rename = "Stats")]
        #[serde(default)]
        pub stats: SsidStats,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct SsidStats {
        #[serde(rename = "BytesReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub bytes_received: u64,
        #[serde(rename = "BytesSent", deserialize_with = "deserialize_string_or_num", default)]
        pub bytes_sent: u64,
        #[serde(rename = "PacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub packets_received: u64,
        #[serde(rename = "PacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub packets_sent: u64,
        #[serde(rename = "BroadcastPacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub broadcast_packets_received: u64,
        #[serde(rename = "BroadcastPacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub broadcast_packets_sent: u64,
        #[serde(rename = "MulticastPacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub multicast_packets_received: u64,
        #[serde(rename = "MulticastPacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub multicast_packets_sent: u64,
        #[serde(rename = "UnicastPacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub unicast_packets_received: u64,
        #[serde(rename = "UnicastPacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub unicast_packets_sent: u64,
        #[serde(rename = "ErrorsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub errors_received: u64,
        #[serde(rename = "ErrorsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub errors_sent: u64,
        #[serde(rename = "DiscardPacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub discard_packets_received: u64,
        #[serde(rename = "DiscardPacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub discard_packets_sent: u64,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct Ethernet {
        #[serde(rename = "Interfaces")]
        #[serde(default)]
        pub interfaces: Vec<EthernetInterface>,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct EthernetInterface {
        #[serde(rename = "Name")]
        #[serde(default)]
        pub name: String,
        #[serde(rename = "Alias")]
        #[serde(default)]
        pub alias: String,
        #[serde(rename = "Status")]
        #[serde(default)]
        pub status: String,
        #[serde(rename = "Enable")]
        pub enable: bool,
        #[serde(rename = "IfcName")]
        #[serde(default)]
        pub ifc_name: String,
        #[serde(rename = "CurrentBitRate")]
        #[serde(default)]
        pub current_bit_rate: i64,
        #[serde(rename = "MACAddress")]
        #[serde(default)]
        pub mac_address: String,
        #[serde(rename = "Diagnostics")]
        #[serde(default)]
        pub diagnostics: EthernetDiagnostics,
        #[serde(rename = "Role")]
        #[serde(default)]
        pub role: String,
        #[serde(rename = "Stats")]
        pub stats: InterfaceStats,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct EthernetDiagnostics {
        #[serde(rename = "CableStatus")]
        #[serde(default)]
        pub cable_status: String,
        #[serde(rename = "CurrentDuplexMode")]
        #[serde(default)]
        pub current_duplex_mode: String,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct Optical {
        #[serde(rename = "Interfaces")]
        #[serde(default)]
        pub interfaces: Vec<OpticalInterface>,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct OpticalInterface {
        #[serde(rename = "Name")]
        #[serde(default)]
        pub name: String,
        #[serde(rename = "Alias")]
        #[serde(default)]
        pub alias: String,
        #[serde(rename = "Status")]
        #[serde(default)]
        pub status: String,
        #[serde(rename = "Alarm")]
        #[serde(default)]
        pub alarm: String,
        #[serde(rename = "IfcName")]
        #[serde(default)]
        pub ifc_name: String,
        #[serde(rename = "OpticalSignalLevel")]
        #[serde(default)]
        pub optical_signal_level: f64,
        #[serde(rename = "Temperature")]
        #[serde(default)]
        pub temperature: f64,
        #[serde(rename = "Voltage")]
        #[serde(default)]
        pub voltage: f64,
        #[serde(rename = "BIASCurrent")]
        #[serde(default)]
        pub bias_current: f64,
        #[serde(rename = "UpperOpticalThreshold")]
        #[serde(default)]
        pub upper_optical_threshold: f64,
        #[serde(rename = "LowerOpticalThreshold")]
        #[serde(default)]
        pub lower_optical_threshold: f64,
        #[serde(rename = "UpperTransmitPowerThreshold")]
        #[serde(default)]
        pub upper_transmit_power_threshold: f64,
        #[serde(rename = "LowerTransmitPowerThreshold")]
        #[serde(default)]
        pub lower_transmit_power_threshold: f64,
        #[serde(rename = "LastChange")]
        #[serde(default)]
        pub last_change: u64,
        #[serde(rename = "OpticalPartNumber")]
        #[serde(default)]
        pub part_number: String,
        #[serde(rename = "OpticalVendorName")]
        #[serde(default)]
        pub vendor_name: String,
        #[serde(rename = "Stats")]
        #[serde(default)]
        pub stats: InterfaceStats,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct InterfaceStats {
        #[serde(rename = "BytesReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub bytes_received: u64,
        #[serde(rename = "BytesSent", deserialize_with = "deserialize_string_or_num", default)]
        pub bytes_sent: u64,
        #[serde(rename = "PacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub packets_received: u64,
        #[serde(rename = "PacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub packets_sent: u64,
        #[serde(rename = "BroadcastPacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub broadcast_packets_received: u64,
        #[serde(rename = "BroadcastPacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub broadcast_packets_sent: u64,
        #[serde(rename = "MulticastPacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub multicast_packets_received: u64,
        #[serde(rename = "MulticastPacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub multicast_packets_sent: u64,
        #[serde(rename = "UnicastPacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub unicast_packets_received: u64,
        #[serde(rename = "UnicastPacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub unicast_packets_sent: u64,
        #[serde(rename = "ErrorsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub errors_received: u64,
        #[serde(rename = "ErrorsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub errors_sent: u64,
        #[serde(rename = "DiscardPacketsReceived", deserialize_with = "deserialize_string_or_num", default)]
        pub discard_packets_received: u64,
        #[serde(rename = "DiscardPacketsSent", deserialize_with = "deserialize_string_or_num", default)]
        pub discard_packets_sent: u64,
    }

    #[derive(Debug, Clone, Default, Deserialize)]
    pub struct ResourceUsage {
        #[serde(rename = "TotalMemory")]
        pub total_memory: i64,
        #[serde(rename = "FreeMemory")]
        pub free_memory: i64,
        #[serde(rename = "AvailableFlashMemory")]
        pub available_flash_memory: i64,
        #[serde(rename = "UsedFlashMemory")]
        pub used_flash_memory: i64,
        #[serde(rename = "CPUUsage")]
        pub cpu_usage: f64,
        #[serde(rename = "LoadAverage")]
        pub load_average: f64,
        #[serde(rename = "LoadAverage5")]
        pub load_average_5: f64,
        #[serde(rename = "LoadAverage15")]
        pub load_average_15: f64,
    }
}

pub use api::{DeviceResponse, Device, Ethernet, Optical, ResourceUsage, Wifi};

#[derive(Clone)]
pub struct Scraper {
    client: Client,
}

impl Scraper {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_device(&self) -> Result<DeviceResponse> {
        let actions = vec![
            crate::client::Action {
                id: 0,
                method: "getValue".to_string(),
                xpath: "Device/DeviceInfo".to_string(),
                parameters: None,
            },
            crate::client::Action {
                id: 1,
                method: "getValue".to_string(),
                xpath: "Device/WiFi".to_string(),
                parameters: None,
            },
            crate::client::Action {
                id: 2,
                method: "getValue".to_string(),
                xpath: "Device/Ethernet".to_string(),
                parameters: None,
            },
            crate::client::Action {
                id: 3,
                method: "getValue".to_string(),
                xpath: "Device/Optical".to_string(),
                parameters: None,
            },
        ];

        let result = self.client.api_request(actions).await?;
        tracing::debug!("get_device response: {}", result);
        
        let reply = result.get("reply").ok_or_else(|| {
            crate::error::Error::InvalidResponse("no reply in response".to_string())
        })?;

        let actions_arr = reply.get("actions").and_then(|a| a.as_array())
            .ok_or_else(|| crate::error::Error::InvalidResponse("no actions in reply".to_string()))?;

        let mut device = Device::default();

        for action in actions_arr {
            let callbacks = action.get("callbacks").and_then(|c| c.as_array())
                .ok_or_else(|| crate::error::Error::InvalidResponse("no callbacks".to_string()))?;

            for callback in callbacks {
                let value = callback.get("parameters")
                    .and_then(|p| p.get("value"));

                let xpath = callback.get("xpath")
                    .and_then(|x| x.as_str())
                    .unwrap_or("");

                if let Some(v) = value {
                    match xpath {
                        "Device/DeviceInfo" => {
                            tracing::debug!("parsing Device/DeviceInfo: {}", v);
                            let mut json_str = v.to_string();
                            json_str = json_str.replace("1-01-01T00:00:00+0000", "0001-01-01T00:00:00+0000");
                            let d: Device = serde_json::from_str(&json_str)?;
                            device.device_info = d.device_info;
                            tracing::debug!("DeviceInfo parsed: {:?}", device.device_info);
                        },
                        "Device/WiFi" => {
                            tracing::debug!("parsing Device/WiFi: {}", v);
                            if let Some(wifi_data) = v.get("WiFi") {
                                match serde_json::from_value::<Wifi>(wifi_data.clone()) {
                                    Ok(wifi) => {
                                        tracing::debug!("WiFi parsed: {} radios, {} ssids", wifi.radios.len(), wifi.ssids.len());
                                        device.wifi = wifi;
                                    },
                                    Err(e) => tracing::debug!("WiFi parse error: {}", e),
                                }
                            } else {
                                match serde_json::from_value::<Wifi>(v.clone()) {
                                    Ok(wifi) => {
                                        tracing::debug!("WiFi parsed: {} radios, {} ssids", wifi.radios.len(), wifi.ssids.len());
                                        device.wifi = wifi;
                                    },
                                    Err(e) => tracing::debug!("WiFi parse error: {}", e),
                                }
                            }
                        },
                        "Device/Ethernet" => {
                            tracing::debug!("parsing Device/Ethernet: {}", v);
                            if let Some(eth_data) = v.get("Ethernet") {
                                match serde_json::from_value::<Ethernet>(eth_data.clone()) {
                                    Ok(eth) => {
                                        tracing::debug!("Ethernet parsed: {} interfaces", eth.interfaces.len());
                                        device.ethernet = eth;
                                    },
                                    Err(e) => tracing::debug!("Ethernet parse error: {}", e),
                                }
                            } else {
                                match serde_json::from_value::<Ethernet>(v.clone()) {
                                    Ok(eth) => {
                                        tracing::debug!("Ethernet parsed: {} interfaces", eth.interfaces.len());
                                        device.ethernet = eth;
                                    },
                                    Err(e) => tracing::debug!("Ethernet parse error: {}", e),
                                }
                            }
                        },
                        "Device/Optical" => {
                            tracing::debug!("parsing Device/Optical: {}", v);
                            if let Some(optical_data) = v.get("Optical") {
                                let interfaces = optical_data.get("Interfaces")
                                    .and_then(|i| i.as_array())
                                    .cloned();
                                if let Some(iface_list) = interfaces {
                                    tracing::debug!("Found {} optical interfaces", iface_list.len());
                                    let opt = Optical {
                                        interfaces: iface_list.into_iter().filter_map(|i| serde_json::from_value(i).ok()).collect()
                                    };
                                    device.optical = opt;
                                } else {
                                    tracing::debug!("No Interfaces found in Optical");
                                }
                            } else {
                                tracing::debug!("No 'Optical' key in response");
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        tracing::debug!("final device: {:?}", device);
        Ok(DeviceResponse { device })
    }

    pub async fn get_resource_usage(&self) -> Result<ResourceUsage> {
        let actions = vec![
            crate::client::Action {
                id: 0,
                method: "getValue".to_string(),
                xpath: "Device/DeviceInfo/MemoryStatus".to_string(),
                parameters: None,
            },
            crate::client::Action {
                id: 1,
                method: "getValue".to_string(),
                xpath: "Device/DeviceInfo/FlashMemoryStatus".to_string(),
                parameters: None,
            },
            crate::client::Action {
                id: 2,
                method: "getValue".to_string(),
                xpath: "Device/DeviceInfo/ProcessStatus/LoadAverage".to_string(),
                parameters: None,
            },
            crate::client::Action {
                id: 3,
                method: "getValue".to_string(),
                xpath: "Device/DeviceInfo/ProcessStatus/CPUUsage".to_string(),
                parameters: None,
            },
        ];

        let result = self.client.api_request(actions).await?;
        let reply = result.get("reply").ok_or_else(|| {
            crate::error::Error::InvalidResponse("no reply in response".to_string())
        })?;

        let actions_arr = reply.get("actions").and_then(|a| a.as_array())
            .ok_or_else(|| crate::error::Error::InvalidResponse("no actions in reply".to_string()))?;

        let mut usage = ResourceUsage::default();

        for action in actions_arr {
            let callbacks = action.get("callbacks").and_then(|c| c.as_array())
                .ok_or_else(|| crate::error::Error::InvalidResponse("no callbacks".to_string()))?;

            for callback in callbacks {
                let value = callback.get("parameters")
                    .and_then(|p| p.get("value"));

                let xpath = callback.get("xpath")
                    .and_then(|x| x.as_str())
                    .unwrap_or("");

                if let Some(v) = value {
                    match xpath {
                        "Device/DeviceInfo/MemoryStatus" => {
                            if let Some(ms) = v.get("MemoryStatus") {
                                if let (Some(total), Some(free)) = (ms.get("Total").and_then(|t| t.as_i64()), ms.get("Free").and_then(|f| f.as_i64())) {
                                    usage.total_memory = total;
                                    usage.free_memory = free;
                                }
                            }
                        },
                        "Device/DeviceInfo/FlashMemoryStatus" => {
                            if let Some(fs) = v.get("FlashMemoryStatus") {
                                if let (Some(total), Some(free)) = (fs.get("Total").and_then(|t| t.as_i64()), fs.get("Free").and_then(|f| f.as_i64())) {
                                    usage.available_flash_memory = free;
                                    usage.used_flash_memory = total - free;
                                }
                            }
                        },
                        "Device/DeviceInfo/ProcessStatus/LoadAverage" => {
                            if let Ok(la) = serde_json::from_value::<serde_json::Value>(v.clone()) {
                                if let Some(load_avg) = la.get("LoadAverage") {
                                    if let Some(load1) = load_avg.get("Load1").and_then(|v| v.as_f64()) {
                                        usage.load_average = load1;
                                    }
                                    if let Some(load5) = load_avg.get("Load5").and_then(|v| v.as_f64()) {
                                        usage.load_average_5 = load5;
                                    }
                                    if let Some(load15) = load_avg.get("Load15").and_then(|v| v.as_f64()) {
                                        usage.load_average_15 = load15;
                                    }
                                }
                            }
                        },
                        "Device/DeviceInfo/ProcessStatus/CPUUsage" => {
                            if let Some(cpu) = v.as_f64().or(v.as_i64().map(|i| i as f64)) {
                                usage.cpu_usage = cpu;
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        Ok(usage)
    }
}
