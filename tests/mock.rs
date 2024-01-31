use pretty_assertions::assert_eq;
use prusa_link_rs::raw_printer::*;
use tokio::test;

/// Creates the base for a mock server, parsing the given url and returning the server, address, port and api key
///
/// Naming convention for the derivative functions is `mock_{path}`
/// where path is the path of the api that is being mocked, with slashes replaced by underscores
/// for example, the path `/api/printer` is mocked by `mock_api_printer`
fn mock_base() -> (mockito::ServerGuard, String, u16, String) {
    let server = mockito::Server::new();

    let url = server.url();
    let url = url.strip_prefix("http://").unwrap();
    let url = url.split_at(url.find(':').unwrap());
    let (address, port) = (
        url.0.to_string(),
        url.1
            .to_string()
            .strip_prefix(':')
            .unwrap()
            .parse::<u16>()
            .unwrap(),
    );

    let api_key = "1234567890";

    (server, address, port, api_key.to_string())
}

fn mock_api_printer() -> (mockito::ServerGuard, mockito::Mock, String, u16, String) {
    let (mut server, address, port, api_key) = mock_base();

    let mock = server
        .mock("GET", "/api/printer")
        .match_header("X-Api-Key", api_key.as_str())
        .with_status(200)
        .with_body(
            r#"{
    "temperature": {
        "tool0": {
            "actual": 220.2,
            "target": 220.0
        },
        "bed": {
            "actual": 69.7,
            "target": 70.0
        }
    },
    "sd": {
        "ready": false
    },
    "state": {
        "text": "Printing",
        "flags": {
            "operational": false,
            "paused": false,
            "printing": true,
            "cancelling": false,
            "pausing": false,
            "sdReady": false,
            "error": false,
            "ready": false,
            "closedOrError": false,
            "finished": false,
            "prepared": false,
            "link_state": "PRINTING"
        }
    },
    "telemetry": {
        "temp-bed": 69.7,
        "temp-nozzle": 220.2,
        "material": " - ",
        "z-height": 16.8,
        "print-speed": 100,
        "axis_x": null,
        "axis_y": null,
        "axis_z": 16.8
    },
    "storage": {
        "local": {
            "free_space": 56813572096,
            "total_space": 61273088000
        },
        "sd_card": null
    }
}"#,
        )
        .create();

    (server, mock, address, port, api_key.to_string())
}

#[test]
async fn get_printer_and_verify() {
    #[allow(unused)]
    let (server, mock, address, port, api_key) = mock_api_printer();

    let printer_builder =
        prusa_link_rs::PrinterBuilder::new(address.to_string(), api_key.to_string());
    let mut printer = printer_builder.port(port.into()).build();

    let raw_printer = printer.get_printer_info().await.unwrap();

    assert_eq!(raw_printer.get_target_nozzle_temp(), 220.0);
    assert_eq!(raw_printer.get_target_bed_temp(), 70.0);
    assert_eq!(raw_printer.get_sd_ready(), false);
    assert_eq!(raw_printer.get_state_text(), "Printing");
    assert_eq!(raw_printer.get_operational(), false);
    assert_eq!(raw_printer.get_paused(), false);
    assert_eq!(raw_printer.get_printing(), true);
    assert_eq!(raw_printer.get_cancelling(), false);
    assert_eq!(raw_printer.get_pausing(), false);
    assert_eq!(raw_printer.get_sd_ready(), false);
    assert_eq!(raw_printer.get_error(), false);
    assert_eq!(raw_printer.get_closed_or_error(), false);
    assert_eq!(raw_printer.get_finished(), false);
    assert_eq!(raw_printer.get_prepared(), false);
    assert_eq!(raw_printer.get_link_state(), "PRINTING");
    assert_eq!(raw_printer.get_bed_temp(), 69.7);
    assert_eq!(raw_printer.get_nozzle_temp(), 220.2);
    assert_eq!(raw_printer.get_material_telemetry(), " - ");
    assert_eq!(raw_printer.get_print_speed_telemetry(), 100.0);
    assert_eq!(raw_printer.get_axis_x_telemetry(), None);
    assert_eq!(raw_printer.get_axis_x_telemetry(), None);
    assert_eq!(raw_printer.get_z_height_telemetry(), 16.8);
    assert_eq!(
        raw_printer.get_local_storage_space(),
        Some(&PrinterStorageInfo {
            free_space: 56813572096,
            total_space: 61273088000,
        })
    );
    assert_eq!(raw_printer.get_sd_storage_space(), None);

    mock.assert();
}

#[test]
async fn autorefresh_and_get_temp() {
    #[allow(unused)]
    let (server, mock, address, port, api_key) = mock_api_printer();

    let printer_builder = prusa_link_rs::PrinterBuilder::new(address, api_key);
    let mut printer = printer_builder.port(port.into()).build();

    assert_eq!(printer.get_nozzle_temp().await.unwrap(), 220.2);
    assert_eq!(printer.get_bed_temp().await.unwrap(), 69.7);
}
