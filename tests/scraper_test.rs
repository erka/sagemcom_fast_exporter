use std::fs;

fn load_test_data(filename: &str) -> String {
    fs::read_to_string(format!("tests/fixtures/{}", filename))
        .expect("failed to read test data file")
}

#[test]
fn test_parse_device_response() {
    let json = load_test_data("device_success_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse device response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    assert_eq!(actions.len(), 4);

    let wifi_action = &actions[1];
    let wifi_callbacks = wifi_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let wifi_value = &wifi_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    let wifi = wifi_value.get("WiFi").expect("no WiFi");
    assert_eq!(
        wifi.get("AccessPoints")
            .expect("no AccessPoints")
            .as_array()
            .expect("AccessPoints is not array")
            .len(),
        1
    );
    assert_eq!(
        wifi.get("Radios")
            .expect("no Radios")
            .as_array()
            .expect("Radios is not array")
            .len(),
        1
    );
    assert_eq!(
        wifi.get("SSIDs")
            .expect("no SSIDs")
            .as_array()
            .expect("SSIDs is not array")
            .len(),
        1
    );

    let device_info_action = &actions[0];
    let device_info_callbacks = device_info_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let device_info_value = &device_info_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    let device_info = device_info_value.get("DeviceInfo").expect("no DeviceInfo");
    assert_eq!(
        device_info
            .get("Manufacturer")
            .expect("no Manufacturer")
            .as_str()
            .expect("Manufacturer is not string"),
        "SagemCom"
    );
    assert_eq!(
        device_info
            .get("ModelName")
            .expect("no ModelName")
            .as_str()
            .expect("ModelName is not string"),
        "Fast5670_ABRV"
    );
}

#[test]
fn test_parse_resource_usage_response() {
    let json = load_test_data("resource_usage_success_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse resource usage response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    assert_eq!(actions.len(), 4);

    let memory_action = &actions[0];
    let memory_callbacks = memory_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let memory_value = &memory_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    let memory_status = memory_value.get("MemoryStatus").expect("no MemoryStatus");
    assert_eq!(
        memory_status
            .get("Total")
            .expect("no Total")
            .as_i64()
            .expect("Total is not i64"),
        504160
    );
    assert_eq!(
        memory_status
            .get("Free")
            .expect("no Free")
            .as_i64()
            .expect("Free is not i64"),
        94440
    );

    let cpu_action = &actions[3];
    let cpu_callbacks = cpu_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let cpu_value = &cpu_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    assert_eq!(cpu_value.as_i64().expect("CPUUsage is not i64"), 5);
}

#[test]
fn test_device_error_response_has_callback_error() {
    let json = load_test_data("device_error_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse device error response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    let action = &actions[0];
    let callbacks = action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let callback = &callbacks[0];
    let result = callback.get("result").expect("no result");
    let code = result
        .get("code")
        .expect("no code field")
        .as_i64()
        .expect("code is not i64");

    assert_eq!(code, 100); // ERROR
}

#[test]
fn test_resource_usage_error_response_has_callback_error() {
    let json = load_test_data("resource_usage_error_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse resource usage error response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    let action = &actions[0];
    let callbacks = action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let callback = &callbacks[0];
    let result = callback.get("result").expect("no result");
    let code = result
        .get("code")
        .expect("no code field")
        .as_i64()
        .expect("code is not i64");

    assert_eq!(code, 100); // ERROR
}

#[test]
fn test_invalid_json_returns_error() {
    let invalid_json = r#"{"invalid": "json""#;
    let result: Result<serde_json::Value, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_auth_success_response() {
    let json = load_test_data("auth_success_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse auth success response JSON");

    let reply = value.get("reply").expect("no reply field");
    let error = reply.get("error").expect("no error field");
    let code = error
        .get("code")
        .expect("no code field")
        .as_i64()
        .expect("code is not i64");

    assert_eq!(code, 16777216); // XMO_REQUEST_NO_ERR
}

#[test]
fn test_ethernet_interface_parsing() {
    let json = load_test_data("device_success_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse device response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    let ethernet_action = &actions[2];
    let ethernet_callbacks = ethernet_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let ethernet_value = &ethernet_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    let ethernet = ethernet_value.get("Ethernet").expect("no Ethernet");
    let interfaces = ethernet
        .get("Interfaces")
        .expect("no Interfaces")
        .as_array()
        .expect("Interfaces is not array");

    assert_eq!(interfaces.len(), 1);

    let iface = &interfaces[0];
    assert_eq!(
        iface
            .get("Alias")
            .expect("no Alias")
            .as_str()
            .expect("Alias is not string"),
        "PHY1"
    );
    assert_eq!(
        iface
            .get("IfcName")
            .expect("no IfcName")
            .as_str()
            .expect("IfcName is not string"),
        "eth0"
    );
    assert_eq!(
        iface
            .get("Status")
            .expect("no Status")
            .as_str()
            .expect("Status is not string"),
        "UP"
    );
}

#[test]
fn test_optical_interface_parsing() {
    let json = load_test_data("device_success_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse device response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    let optical_action = &actions[3];
    let optical_callbacks = optical_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let optical_value = &optical_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    let optical = optical_value.get("Optical").expect("no Optical");
    let interfaces = optical
        .get("Interfaces")
        .expect("no Interfaces")
        .as_array()
        .expect("Interfaces is not array");

    assert_eq!(interfaces.len(), 1);

    let iface = &interfaces[0];
    assert_eq!(
        iface
            .get("Alias")
            .expect("no Alias")
            .as_str()
            .expect("Alias is not string"),
        "OPTICAL0"
    );
    assert_eq!(
        iface
            .get("IfcName")
            .expect("no IfcName")
            .as_str()
            .expect("IfcName is not string"),
        "veip0"
    );
    assert_eq!(
        iface
            .get("Status")
            .expect("no Status")
            .as_str()
            .expect("Status is not string"),
        "UP"
    );
}

#[test]
fn test_wifi_radio_parsing() {
    let json = load_test_data("device_success_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse device response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    let wifi_action = &actions[1];
    let wifi_callbacks = wifi_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let wifi_value = &wifi_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    let wifi = wifi_value.get("WiFi").expect("no WiFi");
    let radios = wifi
        .get("Radios")
        .expect("no Radios")
        .as_array()
        .expect("Radios is not array");

    assert_eq!(radios.len(), 1);

    let radio = &radios[0];
    assert_eq!(
        radio
            .get("Alias")
            .expect("no Alias")
            .as_str()
            .expect("Alias is not string"),
        "RADIO2G4"
    );
    assert_eq!(
        radio
            .get("Name")
            .expect("no Name")
            .as_str()
            .expect("Name is not string"),
        "Device/WiFi/Radios/Radio[RADIO2G4]"
    );
    assert_eq!(
        radio
            .get("Status")
            .expect("no Status")
            .as_str()
            .expect("Status is not string"),
        "UP"
    );
}

#[test]
fn test_wifi_ssid_parsing() {
    let json = load_test_data("device_success_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse device response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    let wifi_action = &actions[1];
    let wifi_callbacks = wifi_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let wifi_value = &wifi_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    let wifi = wifi_value.get("WiFi").expect("no WiFi");
    let ssids = wifi
        .get("SSIDs")
        .expect("no SSIDs")
        .as_array()
        .expect("SSIDs is not array");

    assert_eq!(ssids.len(), 1);

    let ssid = &ssids[0];
    assert_eq!(
        ssid.get("Alias")
            .expect("no Alias")
            .as_str()
            .expect("Alias is not string"),
        "WL_PRIV"
    );
    assert_eq!(
        ssid.get("Status")
            .expect("no Status")
            .as_str()
            .expect("Status is not string"),
        "DOWN"
    );
    assert_eq!(
        ssid.get("LowerLayers")
            .expect("no LowerLayers")
            .as_str()
            .expect("LowerLayers is not string"),
        "Device/WiFi/Radios/Radio[RADIO2G4]"
    );
}

#[test]
fn test_load_average_parsing() {
    let json = load_test_data("resource_usage_success_response.json");
    let value: serde_json::Value =
        serde_json::from_str(&json).expect("failed to parse resource usage response JSON");

    let reply = value.get("reply").expect("no reply field");
    let actions = reply
        .get("actions")
        .expect("no actions field")
        .as_array()
        .expect("actions is not an array");

    let load_avg_action = &actions[2];
    let load_avg_callbacks = load_avg_action
        .get("callbacks")
        .expect("no callbacks")
        .as_array()
        .expect("callbacks is not an array");
    let load_avg_value = &load_avg_callbacks[0]
        .get("parameters")
        .expect("no parameters")
        .get("value")
        .expect("no value");

    let load_avg = load_avg_value.get("LoadAverage").expect("no LoadAverage");
    assert!(
        (load_avg
            .get("Load1")
            .expect("no Load1")
            .as_f64()
            .expect("Load1 is not f64")
            - 2.961426)
            .abs()
            < 0.0001
    );
    assert!(
        (load_avg
            .get("Load5")
            .expect("no Load5")
            .as_f64()
            .expect("Load5 is not f64")
            - 2.86084)
            .abs()
            < 0.0001
    );
    assert!(
        (load_avg
            .get("Load15")
            .expect("no Load15")
            .as_f64()
            .expect("Load15 is not f64")
            - 2.869141)
            .abs()
            < 0.0001
    );
}
