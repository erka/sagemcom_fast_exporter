use crate::scraper::{DeviceResponse, ResourceUsage};
use prometheus::{Encoder, Gauge, GaugeVec, Opts, Registry, TextEncoder};

pub struct Metrics {
    registry: Registry,
    uptime: Gauge,
    reboot_count: Gauge,
    allocator_sys_bytes: Gauge,
    resources_total_memory: Gauge,
    resources_free_memory: Gauge,
    resources_available_flash: Gauge,
    resources_used_flash: Gauge,
    resources_cpu_usage: Gauge,
    resources_load_average: Gauge,
    resources_load_average_5: Gauge,
    resources_load_average_15: Gauge,
    ethernet_bytes_rx: GaugeVec,
    ethernet_bytes_tx: GaugeVec,
    ethernet_packets_rx: GaugeVec,
    ethernet_packets_tx: GaugeVec,
    ethernet_bcast_rx: GaugeVec,
    ethernet_bcast_tx: GaugeVec,
    ethernet_mcast_rx: GaugeVec,
    ethernet_mcast_tx: GaugeVec,
    ethernet_ucast_rx: GaugeVec,
    ethernet_ucast_tx: GaugeVec,
    ethernet_discard_rx: GaugeVec,
    ethernet_errors_rx: GaugeVec,
    ethernet_status: GaugeVec,
    ethernet_bitrate: GaugeVec,
    ethernet_info: GaugeVec,
    optical_bytes_rx: GaugeVec,
    optical_bytes_tx: GaugeVec,
    optical_packets_rx: GaugeVec,
    optical_packets_tx: GaugeVec,
    optical_bcast_rx: GaugeVec,
    optical_bcast_tx: GaugeVec,
    optical_mcast_rx: GaugeVec,
    optical_mcast_tx: GaugeVec,
    optical_ucast_rx: GaugeVec,
    optical_ucast_tx: GaugeVec,
    optical_errors_rx: GaugeVec,
    optical_status: GaugeVec,
    optical_temperature: GaugeVec,
    optical_voltage: GaugeVec,
    optical_signal_level: GaugeVec,
    optical_upper_optical_threshold: GaugeVec,
    optical_lower_optical_threshold: GaugeVec,
    optical_upper_transmit_power_threshold: GaugeVec,
    optical_lower_transmit_power_threshold: GaugeVec,
    optical_bias_current: GaugeVec,
    optical_last_change: GaugeVec,
    optical_info: GaugeVec,
    wifi_radio_status: GaugeVec,
    wifi_radio_channel: GaugeVec,
    wifi_radio_noise: GaugeVec,
    wifi_radio_tx_power: GaugeVec,
    wifi_radio_tx_power_max: GaugeVec,
    wifi_radio_bandwidth: GaugeVec,
    wifi_radio_max_bitrate: GaugeVec,
    wifi_radio_last_change: GaugeVec,
    wifi_radio_info: GaugeVec,
    wifi_ssid_status: GaugeVec,
    wifi_ssid_info: GaugeVec,
    wifi_ssid_bytes_rx: GaugeVec,
    wifi_ssid_bytes_tx: GaugeVec,
    wifi_ssid_packets_rx: GaugeVec,
    wifi_ssid_packets_tx: GaugeVec,
    wifi_ssid_bcast_rx: GaugeVec,
    wifi_ssid_bcast_tx: GaugeVec,
    wifi_ssid_mcast_rx: GaugeVec,
    wifi_ssid_mcast_tx: GaugeVec,
    wifi_ssid_ucast_rx: GaugeVec,
    wifi_ssid_ucast_tx: GaugeVec,
    wifi_ssid_discard_rx: GaugeVec,
    wifi_ssid_discard_tx: GaugeVec,
    wifi_ssid_errors_rx: GaugeVec,
    wifi_ssid_errors_tx: GaugeVec,
    device_info: GaugeVec,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let uptime = Gauge::new(
            "sagemcom_fast_system_uptime_seconds",
            "System uptime in seconds",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(uptime.clone()))
            .expect("failed to create metric");

        let reboot_count = Gauge::new("sagemcom_fast_system_reboot_count", "Number of reboots")
            .expect("failed to create metric");
        registry
            .register(Box::new(reboot_count.clone()))
            .expect("failed to create metric");

        let allocator_sys_bytes = Gauge::new(
            "sagemcom_fast_process_allocator_sys_bytes",
            "Allocator memory in bytes",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(allocator_sys_bytes.clone()))
            .expect("failed to create metric");

        let resources_total_memory = Gauge::new(
            "sagemcom_fast_resources_total_memory_bytes",
            "Total memory available to the system",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(resources_total_memory.clone()))
            .expect("failed to create metric");

        let resources_free_memory = Gauge::new(
            "sagemcom_fast_resources_free_memory_bytes",
            "Free memory available to the system",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(resources_free_memory.clone()))
            .expect("failed to create metric");

        let resources_available_flash = Gauge::new(
            "sagemcom_fast_resources_available_flash_memory_bytes",
            "Available flash memory",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(resources_available_flash.clone()))
            .expect("failed to create metric");

        let resources_used_flash = Gauge::new(
            "sagemcom_fast_resources_used_flash_memory_bytes",
            "Used flash memory",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(resources_used_flash.clone()))
            .expect("failed to create metric");

        let resources_cpu_usage =
            Gauge::new("sagemcom_fast_resources_cpu_usage", "CPU usage percentage")
                .expect("failed to create metric");
        registry
            .register(Box::new(resources_cpu_usage.clone()))
            .expect("failed to create metric");

        let resources_load_average = Gauge::new(
            "sagemcom_fast_resources_load_average_1m",
            "Load average 1 minute",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(resources_load_average.clone()))
            .expect("failed to create metric");

        let resources_load_average_5 = Gauge::new(
            "sagemcom_fast_resources_load_average_5m",
            "Load average 5 minutes",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(resources_load_average_5.clone()))
            .expect("failed to create metric");

        let resources_load_average_15 = Gauge::new(
            "sagemcom_fast_resources_load_average_15m",
            "Load average 15 minutes",
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(resources_load_average_15.clone()))
            .expect("failed to create metric");

        let ethernet_bytes_rx = GaugeVec::new(
            Opts::new("sagemcom_fast_ethernet_rx_bytes_total", "Bytes received"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_bytes_rx.clone()))
            .expect("failed to create metric");

        let ethernet_bytes_tx = GaugeVec::new(
            Opts::new("sagemcom_fast_ethernet_tx_bytes_total", "Bytes transmitted"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_bytes_tx.clone()))
            .expect("failed to create metric");

        let ethernet_packets_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_rx_packets_total",
                "Packets received",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_packets_rx.clone()))
            .expect("failed to create metric");

        let ethernet_packets_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_tx_packets_total",
                "Packets transmitted",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_packets_tx.clone()))
            .expect("failed to create metric");

        let ethernet_errors_rx = GaugeVec::new(
            Opts::new("sagemcom_fast_ethernet_rx_errors_total", "Receive errors"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_errors_rx.clone()))
            .expect("failed to create metric");

        let optical_bytes_rx = GaugeVec::new(
            Opts::new("sagemcom_fast_optical_rx_bytes_total", "Bytes received"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_bytes_rx.clone()))
            .expect("failed to create metric");

        let optical_bytes_tx = GaugeVec::new(
            Opts::new("sagemcom_fast_optical_tx_bytes_total", "Bytes transmitted"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_bytes_tx.clone()))
            .expect("failed to create metric");

        let optical_packets_rx = GaugeVec::new(
            Opts::new("sagemcom_fast_optical_rx_packets_total", "Packets received"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_packets_rx.clone()))
            .expect("failed to create metric");

        let optical_packets_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_tx_packets_total",
                "Packets transmitted",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_packets_tx.clone()))
            .expect("failed to create metric");

        let optical_errors_rx = GaugeVec::new(
            Opts::new("sagemcom_fast_optical_rx_errors_total", "Receive errors"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_errors_rx.clone()))
            .expect("failed to create metric");

        let wifi_radio_status = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_radio_status", "Status of wifi radio"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_status.clone()))
            .expect("failed to create metric");

        let wifi_radio_channel = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_radio_channel", "Channel of wifi radio"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_channel.clone()))
            .expect("failed to create metric");

        let wifi_radio_noise = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_radio_noise_dbm", "Noise of wifi radio"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_noise.clone()))
            .expect("failed to create metric");

        let wifi_radio_tx_power = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_radio_transmit_power_percentage",
                "Transmit power percentage",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_tx_power.clone()))
            .expect("failed to create metric");

        let wifi_radio_tx_power_max = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_radio_transmit_power_max_dbm",
                "Max transmit power",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_tx_power_max.clone()))
            .expect("failed to create metric");

        let wifi_radio_bandwidth = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_radio_bandwidth_hz", "Bandwidth"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_bandwidth.clone()))
            .expect("failed to create metric");

        let wifi_radio_max_bitrate = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_radio_max_bit_rate_bytes_per_second",
                "Max bit rate",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_max_bitrate.clone()))
            .expect("failed to create metric");

        let wifi_radio_last_change = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_radio_last_change_timestamp",
                "Last change timestamp",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_last_change.clone()))
            .expect("failed to create metric");

        let wifi_radio_info = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_radio_info", "Radio info"),
            &[
                "name",
                "alias",
                "regulatory_domain",
                "supported_standards",
                "supported_bandwidth",
            ],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_radio_info.clone()))
            .expect("failed to create metric");

        let optical_bcast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_broadcast_rx_packets_total",
                "Broadcast packets received",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_bcast_rx.clone()))
            .expect("failed to create metric");

        let optical_bcast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_broadcast_tx_packets_total",
                "Broadcast packets transmitted",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_bcast_tx.clone()))
            .expect("failed to create metric");

        let optical_mcast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_multicast_rx_packets_total",
                "Multicast packets received",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_mcast_rx.clone()))
            .expect("failed to create metric");

        let optical_mcast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_multicast_tx_packets_total",
                "Multicast packets transmitted",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_mcast_tx.clone()))
            .expect("failed to create metric");

        let optical_ucast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_unicast_rx_packets_total",
                "Unicast packets received",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_ucast_rx.clone()))
            .expect("failed to create metric");

        let optical_ucast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_unicast_tx_packets_total",
                "Unicast packets transmitted",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_ucast_tx.clone()))
            .expect("failed to create metric");

        let optical_status = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_status",
                "Status of optical interface",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_status.clone()))
            .expect("failed to create metric");

        let optical_temperature = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_temperature_degrees_celsius",
                "Temperature",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_temperature.clone()))
            .expect("failed to create metric");

        let optical_voltage = GaugeVec::new(
            Opts::new("sagemcom_fast_optical_voltage_volts", "Voltage"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_voltage.clone()))
            .expect("failed to create metric");

        let optical_signal_level = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_signal_level_dbm",
                "Optical signal level",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_signal_level.clone()))
            .expect("failed to create metric");

        let optical_upper_optical_threshold = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_upper_optical_threshold",
                "Upper optical threshold",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_upper_optical_threshold.clone()))
            .expect("failed to create metric");

        let optical_lower_optical_threshold = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_lower_optical_threshold",
                "Lower optical threshold",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_lower_optical_threshold.clone()))
            .expect("failed to create metric");

        let optical_upper_transmit_power_threshold = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_upper_transmit_power_threshold",
                "Upper transmit power threshold",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_upper_transmit_power_threshold.clone()))
            .expect("failed to create metric");

        let optical_lower_transmit_power_threshold = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_lower_transmit_power_threshold",
                "Lower transmit power threshold",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_lower_transmit_power_threshold.clone()))
            .expect("failed to create metric");

        let optical_bias_current = GaugeVec::new(
            Opts::new("sagemcom_fast_optical_bias_current", "Bias current (mA)"),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_bias_current.clone()))
            .expect("failed to create metric");

        let optical_last_change = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_optical_last_change_timestamp",
                "Last change timestamp",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_last_change.clone()))
            .expect("failed to create metric");

        let ethernet_bcast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_broadcast_rx_packets_total",
                "Broadcast packets received",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_bcast_rx.clone()))
            .expect("failed to create metric");

        let ethernet_bcast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_broadcast_tx_packets_total",
                "Broadcast packets transmitted",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_bcast_tx.clone()))
            .expect("failed to create metric");

        let ethernet_mcast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_multicast_rx_packets_total",
                "Multicast packets received",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_mcast_rx.clone()))
            .expect("failed to create metric");

        let ethernet_mcast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_multicast_tx_packets_total",
                "Multicast packets transmitted",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_mcast_tx.clone()))
            .expect("failed to create metric");

        let ethernet_ucast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_unicast_rx_packets_total",
                "Unicast packets received",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_ucast_rx.clone()))
            .expect("failed to create metric");

        let ethernet_ucast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_unicast_tx_packets_total",
                "Unicast packets transmitted",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_ucast_tx.clone()))
            .expect("failed to create metric");

        let ethernet_discard_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_rx_discarded_packets_total",
                "Discarded packets received",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_discard_rx.clone()))
            .expect("failed to create metric");

        let ethernet_status = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_status",
                "Status of ethernet interface",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_status.clone()))
            .expect("failed to create metric");

        let ethernet_bitrate = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_ethernet_bitrate",
                "Bit rate of ethernet interface (bits per second)",
            ),
            &["name", "alias"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_bitrate.clone()))
            .expect("failed to create metric");

        let ethernet_info = GaugeVec::new(
            Opts::new("sagemcom_fast_ethernet_info", "Ethernet interface info"),
            &[
                "name",
                "alias",
                "cable_status",
                "current_duplex_mode",
                "mac_address",
                "role",
                "status",
            ],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(ethernet_info.clone()))
            .expect("failed to create metric");

        let optical_info = GaugeVec::new(
            Opts::new("sagemcom_fast_optical_info", "Optical interface info"),
            &[
                "name",
                "alias",
                "alarm",
                "part_number",
                "vendor_name",
                "status",
            ],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(optical_info.clone()))
            .expect("failed to create metric");

        let wifi_ssid_info = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_ssid_info", "SSID info"),
            &["name", "alias", "ssid", "radio", "status", "mac_address"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_info.clone()))
            .expect("failed to create metric");

        let wifi_ssid_status = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_ssid_status", "Status of wifi SSID"),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_status.clone()))
            .expect("failed to create metric");

        let wifi_ssid_bytes_rx = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_ssid_rx_bytes_total", "Bytes received"),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_bytes_rx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_bytes_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_tx_bytes_total",
                "Bytes transmitted",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_bytes_tx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_packets_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_rx_packets_total",
                "Packets received",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_packets_rx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_packets_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_tx_packets_total",
                "Packets transmitted",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_packets_tx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_bcast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_broadcast_rx_packets_total",
                "Broadcast packets received",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_bcast_rx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_bcast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_broadcast_tx_packets_total",
                "Broadcast packets transmitted",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_bcast_tx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_mcast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_multicast_rx_packets_total",
                "Multicast packets received",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_mcast_rx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_mcast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_multicast_tx_packets_total",
                "Multicast packets transmitted",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_mcast_tx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_ucast_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_unicast_rx_packets_total",
                "Unicast packets received",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_ucast_rx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_ucast_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_unicast_tx_packets_total",
                "Unicast packets transmitted",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_ucast_tx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_discard_rx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_rx_discarded_packets_total",
                "Discarded packets received",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_discard_rx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_discard_tx = GaugeVec::new(
            Opts::new(
                "sagemcom_fast_wifi_ssid_tx_discarded_packets_total",
                "Discarded packets transmitted",
            ),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_discard_tx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_errors_rx = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_ssid_rx_errors_total", "Receive errors"),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_errors_rx.clone()))
            .expect("failed to create metric");

        let wifi_ssid_errors_tx = GaugeVec::new(
            Opts::new("sagemcom_fast_wifi_ssid_tx_errors_total", "Transmit errors"),
            &["name", "alias", "ssid", "radio"],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(wifi_ssid_errors_tx.clone()))
            .expect("failed to create metric");

        let device_info = GaugeVec::new(
            Opts::new("sagemcom_fast_system_device_info", "Device information"),
            &[
                "additional_hardware_version",
                "additional_software_version",
                "backup_software_version",
                "country",
                "description",
                "external_firmware_version",
                "gui_api_version",
                "gui_firmware_version",
                "hardware_version",
                "internal_firmware_version",
                "mac_address",
                "manufacturer",
                "manufacturer_oui",
                "mode",
                "model_name",
                "model_number",
                "ont_serial_number",
                "product_class",
                "provisioning_code",
                "router_name",
                "serial_number",
                "software_version",
                "spec_version",
            ],
        )
        .expect("failed to create metric");
        registry
            .register(Box::new(device_info.clone()))
            .expect("failed to create metric");

        Self {
            registry,
            uptime,
            reboot_count,
            allocator_sys_bytes,
            resources_total_memory,
            resources_free_memory,
            resources_available_flash,
            resources_used_flash,
            resources_cpu_usage,
            resources_load_average,
            resources_load_average_5,
            resources_load_average_15,
            ethernet_bytes_rx,
            ethernet_bytes_tx,
            ethernet_packets_rx,
            ethernet_packets_tx,
            ethernet_bcast_rx,
            ethernet_bcast_tx,
            ethernet_mcast_rx,
            ethernet_mcast_tx,
            ethernet_ucast_rx,
            ethernet_ucast_tx,
            ethernet_discard_rx,
            ethernet_errors_rx,
            ethernet_status,
            ethernet_bitrate,
            ethernet_info,
            optical_bytes_rx,
            optical_bytes_tx,
            optical_packets_rx,
            optical_packets_tx,
            optical_bcast_rx,
            optical_bcast_tx,
            optical_mcast_rx,
            optical_mcast_tx,
            optical_ucast_rx,
            optical_ucast_tx,
            optical_errors_rx,
            optical_status,
            optical_temperature,
            optical_voltage,
            optical_signal_level,
            optical_upper_optical_threshold,
            optical_lower_optical_threshold,
            optical_upper_transmit_power_threshold,
            optical_lower_transmit_power_threshold,
            optical_bias_current,
            optical_last_change,
            optical_info,
            wifi_radio_status,
            wifi_radio_channel,
            wifi_radio_noise,
            wifi_radio_tx_power,
            wifi_radio_tx_power_max,
            wifi_radio_bandwidth,
            wifi_radio_max_bitrate,
            wifi_radio_last_change,
            wifi_radio_info,
            wifi_ssid_status,
            wifi_ssid_info,
            wifi_ssid_bytes_rx,
            wifi_ssid_bytes_tx,
            wifi_ssid_packets_rx,
            wifi_ssid_packets_tx,
            wifi_ssid_bcast_rx,
            wifi_ssid_bcast_tx,
            wifi_ssid_mcast_rx,
            wifi_ssid_mcast_tx,
            wifi_ssid_ucast_rx,
            wifi_ssid_ucast_tx,
            wifi_ssid_discard_rx,
            wifi_ssid_discard_tx,
            wifi_ssid_errors_rx,
            wifi_ssid_errors_tx,
            device_info,
        }
    }

    fn status_to_number(status: &str) -> f64 {
        match status {
            "UP" => 0.0,
            "DOWN" => 1.0,
            "UNKNOWN" => 2.0,
            "DORMANT" => 3.0,
            "NOTPRESENT" => 4.0,
            "LOWERLAYERDOWN" => 5.0,
            "ERROR" => 6.0,
            _ => 2.0,
        }
    }

    fn parse_bandwidth(s: &str) -> f64 {
        if s.ends_with("MHz") {
            s.trim_end_matches("MHz").parse().unwrap_or(0.0) * 1_000_000.0
        } else if s.ends_with("GHz") {
            s.trim_end_matches("GHz").parse().unwrap_or(0.0) * 1_000_000_000.0
        } else {
            0.0
        }
    }

    pub fn update(&mut self, device: &DeviceResponse, resources: &ResourceUsage) {
        self.uptime.set(device.device.device_info.up_time as f64);
        self.reboot_count
            .set(device.device.device_info.reboot_count as f64);
        self.allocator_sys_bytes.set(0.0);
        self.resources_total_memory
            .set((resources.total_memory * 1024) as f64);
        self.resources_free_memory
            .set((resources.free_memory * 1024) as f64);
        self.resources_available_flash
            .set((resources.available_flash_memory * 1024) as f64);
        self.resources_used_flash
            .set((resources.used_flash_memory * 1024) as f64);
        self.resources_cpu_usage.set(resources.cpu_usage / 100.0);
        self.resources_load_average.set(resources.load_average);
        self.resources_load_average_5.set(resources.load_average_5);
        self.resources_load_average_15
            .set(resources.load_average_15);

        for iface in &device.device.ethernet.interfaces {
            if !iface.enable {
                continue;
            }
            let labels = [iface.ifc_name.as_str(), iface.alias.as_str()];
            self.ethernet_bytes_rx
                .with_label_values(&labels)
                .set(iface.stats.bytes_received as f64);
            self.ethernet_bytes_tx
                .with_label_values(&labels)
                .set(iface.stats.bytes_sent as f64);
            self.ethernet_packets_rx
                .with_label_values(&labels)
                .set(iface.stats.packets_received as f64);
            self.ethernet_packets_tx
                .with_label_values(&labels)
                .set(iface.stats.packets_sent as f64);
            self.ethernet_errors_rx
                .with_label_values(&labels)
                .set(iface.stats.errors_received as f64);
            self.ethernet_bcast_rx
                .with_label_values(&labels)
                .set(iface.stats.broadcast_packets_received as f64);
            self.ethernet_bcast_tx
                .with_label_values(&labels)
                .set(iface.stats.broadcast_packets_sent as f64);
            self.ethernet_mcast_rx
                .with_label_values(&labels)
                .set(iface.stats.multicast_packets_received as f64);
            self.ethernet_mcast_tx
                .with_label_values(&labels)
                .set(iface.stats.multicast_packets_sent as f64);
            self.ethernet_ucast_rx
                .with_label_values(&labels)
                .set(iface.stats.unicast_packets_received as f64);
            self.ethernet_ucast_tx
                .with_label_values(&labels)
                .set(iface.stats.unicast_packets_sent as f64);
            self.ethernet_discard_rx
                .with_label_values(&labels)
                .set(iface.stats.discard_packets_received as f64);
            self.ethernet_status
                .with_label_values(&labels)
                .set(Self::status_to_number(&iface.status));
            self.ethernet_bitrate
                .with_label_values(&labels)
                .set(iface.current_bit_rate as f64 * 1_000_000.0);
            self.ethernet_info
                .with_label_values(&[
                    iface.ifc_name.as_str(),
                    iface.alias.as_str(),
                    iface.diagnostics.cable_status.as_str(),
                    iface.diagnostics.current_duplex_mode.as_str(),
                    iface.mac_address.as_str(),
                    iface.role.as_str(),
                    iface.status.as_str(),
                ])
                .set(1.0);
        }

        for iface in &device.device.optical.interfaces {
            let labels = [iface.ifc_name.as_str(), iface.alias.as_str()];
            self.optical_bytes_rx
                .with_label_values(&labels)
                .set(iface.stats.bytes_received as f64);
            self.optical_bytes_tx
                .with_label_values(&labels)
                .set(iface.stats.bytes_sent as f64);
            self.optical_packets_rx
                .with_label_values(&labels)
                .set(iface.stats.packets_received as f64);
            self.optical_packets_tx
                .with_label_values(&labels)
                .set(iface.stats.packets_sent as f64);
            self.optical_errors_rx
                .with_label_values(&labels)
                .set(iface.stats.errors_received as f64);
            self.optical_bcast_rx
                .with_label_values(&labels)
                .set(iface.stats.broadcast_packets_received as f64);
            self.optical_bcast_tx
                .with_label_values(&labels)
                .set(iface.stats.broadcast_packets_sent as f64);
            self.optical_mcast_rx
                .with_label_values(&labels)
                .set(iface.stats.multicast_packets_received as f64);
            self.optical_mcast_tx
                .with_label_values(&labels)
                .set(iface.stats.multicast_packets_sent as f64);
            self.optical_ucast_rx
                .with_label_values(&labels)
                .set(iface.stats.unicast_packets_received as f64);
            self.optical_ucast_tx
                .with_label_values(&labels)
                .set(iface.stats.unicast_packets_sent as f64);
            self.optical_status
                .with_label_values(&labels)
                .set(Self::status_to_number(&iface.status));
            self.optical_temperature
                .with_label_values(&labels)
                .set(iface.temperature / 1000.0);
            self.optical_voltage
                .with_label_values(&labels)
                .set(iface.voltage / 1_000_000.0);
            self.optical_signal_level
                .with_label_values(&labels)
                .set(iface.optical_signal_level / 1000.0);
            self.optical_upper_optical_threshold
                .with_label_values(&labels)
                .set(iface.upper_optical_threshold / 1000.0);
            self.optical_lower_optical_threshold
                .with_label_values(&labels)
                .set(iface.lower_optical_threshold / 1000.0);
            self.optical_upper_transmit_power_threshold
                .with_label_values(&labels)
                .set(iface.upper_transmit_power_threshold / 1000.0);
            self.optical_lower_transmit_power_threshold
                .with_label_values(&labels)
                .set(iface.lower_transmit_power_threshold / 1000.0);
            self.optical_bias_current
                .with_label_values(&labels)
                .set(iface.bias_current);
            self.optical_last_change
                .with_label_values(&labels)
                .set(iface.last_change as f64);
            self.optical_info
                .with_label_values(&[
                    iface.ifc_name.as_str(),
                    iface.alias.as_str(),
                    iface.alarm.as_str(),
                    iface.part_number.as_str(),
                    iface.vendor_name.as_str(),
                    iface.status.as_str(),
                ])
                .set(1.0);
        }

        let mut radio_name_to_ifc = std::collections::HashMap::new();
        for radio in &device.device.wifi.radios {
            radio_name_to_ifc.insert(radio.name.clone(), radio.ifc_name.clone());
        }

        for radio in &device.device.wifi.radios {
            let labels = [radio.ifc_name.as_str(), radio.alias.as_str()];
            self.wifi_radio_status
                .with_label_values(&labels)
                .set(Self::status_to_number(&radio.status));
            self.wifi_radio_channel
                .with_label_values(&labels)
                .set(radio.channel as f64);
            self.wifi_radio_noise
                .with_label_values(&labels)
                .set(radio.stats.noise);
            self.wifi_radio_tx_power
                .with_label_values(&labels)
                .set(radio.transmit_power);
            self.wifi_radio_tx_power_max
                .with_label_values(&labels)
                .set(radio.transmit_power_max);
            self.wifi_radio_bandwidth
                .with_label_values(&labels)
                .set(Self::parse_bandwidth(
                    &radio.current_operating_channel_bandwidth,
                ));
            self.wifi_radio_max_bitrate
                .with_label_values(&labels)
                .set(radio.max_bit_rate as f64);
            self.wifi_radio_last_change
                .with_label_values(&labels)
                .set(radio.last_change as f64);
            self.wifi_radio_info
                .with_label_values(&[
                    radio.ifc_name.as_str(),
                    radio.alias.as_str(),
                    radio.regulatory_domain.as_str(),
                    radio.supported_standards.as_str(),
                    radio.supported_channel_bandwidth.as_str(),
                ])
                .set(1.0);
        }

        for ssid in &device.device.wifi.ssids {
            let radio_ifc = radio_name_to_ifc
                .get(&ssid.lower_layers)
                .cloned()
                .unwrap_or_default();
            let labels = [
                ssid.ifc_name.as_str(),
                ssid.alias.as_str(),
                ssid.ssid.as_str(),
                radio_ifc.as_str(),
            ];
            self.wifi_ssid_bytes_rx
                .with_label_values(&labels)
                .set(ssid.stats.bytes_received as f64);
            self.wifi_ssid_bytes_tx
                .with_label_values(&labels)
                .set(ssid.stats.bytes_sent as f64);
            self.wifi_ssid_packets_rx
                .with_label_values(&labels)
                .set(ssid.stats.packets_received as f64);
            self.wifi_ssid_packets_tx
                .with_label_values(&labels)
                .set(ssid.stats.packets_sent as f64);
            self.wifi_ssid_status
                .with_label_values(&labels)
                .set(Self::status_to_number(&ssid.status));
            self.wifi_ssid_info
                .with_label_values(&[
                    ssid.ifc_name.as_str(),
                    ssid.alias.as_str(),
                    ssid.ssid.as_str(),
                    radio_ifc.as_str(),
                    ssid.status.as_str(),
                    ssid.mac_address.as_str(),
                ])
                .set(1.0);
            self.wifi_ssid_bcast_rx
                .with_label_values(&labels)
                .set(ssid.stats.broadcast_packets_received as f64);
            self.wifi_ssid_bcast_tx
                .with_label_values(&labels)
                .set(ssid.stats.broadcast_packets_sent as f64);
            self.wifi_ssid_mcast_rx
                .with_label_values(&labels)
                .set(ssid.stats.multicast_packets_received as f64);
            self.wifi_ssid_mcast_tx
                .with_label_values(&labels)
                .set(ssid.stats.multicast_packets_sent as f64);
            self.wifi_ssid_ucast_rx
                .with_label_values(&labels)
                .set(ssid.stats.unicast_packets_received as f64);
            self.wifi_ssid_ucast_tx
                .with_label_values(&labels)
                .set(ssid.stats.unicast_packets_sent as f64);
            self.wifi_ssid_discard_rx
                .with_label_values(&labels)
                .set(ssid.stats.discard_packets_received as f64);
            self.wifi_ssid_discard_tx
                .with_label_values(&labels)
                .set(ssid.stats.discard_packets_sent as f64);
            self.wifi_ssid_errors_rx
                .with_label_values(&labels)
                .set(ssid.stats.errors_received as f64);
            self.wifi_ssid_errors_tx
                .with_label_values(&labels)
                .set(ssid.stats.errors_sent as f64);
        }

        self.device_info
            .with_label_values(&[
                device
                    .device
                    .device_info
                    .additional_hardware_version
                    .as_str(),
                device
                    .device
                    .device_info
                    .additional_software_version
                    .as_str(),
                device.device.device_info.backup_software_version.as_str(),
                device.device.device_info.country.as_str(),
                device.device.device_info.description.as_str(),
                device.device.device_info.external_firmware_version.as_str(),
                device.device.device_info.gui_api_version.as_str(),
                device.device.device_info.gui_firmware_version.as_str(),
                device.device.device_info.hardware_version.as_str(),
                device.device.device_info.internal_firmware_version.as_str(),
                device.device.device_info.mac_address.as_str(),
                device.device.device_info.manufacturer.as_str(),
                device.device.device_info.manufacturer_oui.as_str(),
                device.device.device_info.mode.as_str(),
                device.device.device_info.model_name.as_str(),
                device.device.device_info.model_number.as_str(),
                device.device.device_info.ont_serial_number.as_str(),
                device.device.device_info.product_class.as_str(),
                device.device.device_info.provisioning_code.as_str(),
                device.device.device_info.router_name.as_str(),
                device.device.device_info.serial_number.as_str(),
                device.device.device_info.software_version.as_str(),
                device.device.device_info.spec_version.as_str(),
            ])
            .set(1.0);
    }

    pub fn gather(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder
            .encode(&metric_families, &mut buffer)
            .expect("failed to create metric");
        String::from_utf8(buffer).expect("failed to create metric")
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_device_response() -> DeviceResponse {
        let payload = json!({
            "Device": {
                "DeviceInfo": {
                    "UpTime": 100,
                    "RebootCount": 2,
                    "MemoryStatus": { "Total": 1024, "Free": 256 },
                    "ProductClass": "Fast",
                    "SerialNumber": "SN123",
                    "ModelName": "Fast5670",
                    "Manufacturer": "Sagemcom",
                    "SoftwareVersion": "1.0.0",
                    "HardwareVersion": "A1",
                    "MACAddress": "00:11:22:33:44:55",
                    "Country": "PT",
                    "Description": "Test device"
                },
                "Ethernet": {
                    "Interfaces": [{
                        "Name": "eth0",
                        "Alias": "LAN1",
                        "Enable": true,
                        "Stats": {
                            "BytesReceived": 100,
                            "BytesSent": 200,
                            "PacketsReceived": 10,
                            "PacketsSent": 20,
                            "ErrorsReceived": 1
                        }
                    }]
                },
                "Optical": {
                    "Interfaces": [{
                        "Name": "opt0",
                        "Alias": "WAN",
                        "Stats": {
                            "BytesReceived": 300,
                            "BytesSent": 400,
                            "PacketsReceived": 30,
                            "PacketsSent": 40,
                            "ErrorsReceived": 2
                        }
                    }]
                },
                "WiFi": {
                    "SSIDs": [{
                        "Name": "ssid0",
                        "Alias": "SSID0",
                        "SSID": "MyWifi",
                        "Enable": true,
                        "Stats": {
                            "BytesReceived": 500,
                            "BytesSent": 600,
                            "PacketsReceived": 50,
                            "PacketsSent": 60
                        }
                    }]
                }
            }
        });

        serde_json::from_value(payload).expect("device response")
    }

    #[test]
    fn update_does_not_double_count_interface_counters() {
        let device = sample_device_response();
        let resources = ResourceUsage {
            total_memory: 1024,
            free_memory: 256,
            available_flash_memory: 100,
            used_flash_memory: 50,
            cpu_usage: 12.5,
            load_average: 1.0,
            load_average_5: 2.0,
            load_average_15: 3.0,
        };

        let mut metrics = Metrics::new();

        metrics.update(&device, &resources);
        let eth_rx_first = metrics
            .ethernet_bytes_rx
            .with_label_values(&["eth0", "LAN1"])
            .get();

        metrics.update(&device, &resources);
        let eth_rx_second = metrics
            .ethernet_bytes_rx
            .with_label_values(&["eth0", "LAN1"])
            .get();

        assert_eq!(eth_rx_first, eth_rx_second);
    }
}
