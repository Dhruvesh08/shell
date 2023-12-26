use mecha_network_ctl::wireless_network::WirelessNetworkControl;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct NetworkManager {}

const NETWORK_CONNECT_SUCCESS_MESSAGE: &str = "WiFi connection successful";
const NETWORK_CONNECT_FAILURE_MESSAGE: &str = "WiFi connection failed";
const NETWORK_REMOVAL_SUCCESS_MESSAGE: &str = "WiFi network removed successfully";
const NETWORK_REMOVAL_FAILURE_MESSAGE: &str = "WiFi network removal failed";

#[allow(non_snake_case)]
pub mod networkmanager {
    tonic::include_proto!("networkmanager");
}

pub use networkmanager::{
    network_manager_service_server::{NetworkManagerService, NetworkManagerServiceServer},
    Empty, NetworkResult, RemoveNetworkRequest, RemoveNetworkResponse, ScanResult, ScanResults,
    WifiConnectRequest, WifiConnectResponse, WifiStatusResponse,
};

use self::networkmanager::NetworkResults;

trait ResponseMessage {
    fn set_success(&mut self, success: bool);
    fn set_message(&mut self, message: String);
}

impl ResponseMessage for WifiConnectResponse {
    fn set_success(&mut self, success: bool) {
        self.success = success;
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }
}

impl ResponseMessage for RemoveNetworkResponse {
    fn set_success(&mut self, success: bool) {
        self.success = success;
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }
}

impl NetworkManager {
    fn handle_response<T: ResponseMessage>(
        &self,
        result: Result<(), &str>,
        response: &mut T,
        success_message: &str,
        failure_message: &str,
    ) {
        match result {
            Ok(_) => {
                response.set_success(true);
                response.set_message(success_message.to_string());
            }
            Err(_) => {
                response.set_success(false);
                response.set_message(failure_message.to_string());
            }
        }
    }

    async fn connect_to_wifi(&self, ssid: &str, psk: &str) -> Result<(), &str> {
        let connect_wireless_network_list =
            WirelessNetworkControl::connect_wireless_network(ssid, psk).await;

        match connect_wireless_network_list {
            Ok(_) => Ok(()),
            Err(_) => Err(NETWORK_CONNECT_FAILURE_MESSAGE),
        }
    }

    async fn remove_wifi_network(&self, network_id: usize) -> Result<(), &str> {
        let remove_network = WirelessNetworkControl::remove_wireless_network(network_id).await;

        match remove_network {
            Ok(_) => Ok(()),
            Err(_) => Err(NETWORK_REMOVAL_FAILURE_MESSAGE),
        }
    }
}

#[tonic::async_trait]
impl NetworkManagerService for NetworkManager {
    async fn scan_wireless_network(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ScanResults>, Status> {
        // Implement your async get_wifi logic here
        let mut scan_results = ScanResults::default();

        log::info!("Starting All Wifi List Function");
        let wifi_service = WirelessNetworkControl::new();

        //get wifi list from mecha_edge_sdk
        // Attempt to get the wifi list from mecha_edge_sdk and handle errors.
        let wireless_network_list = match wifi_service.scan_wireless_network().await {
            Ok(wireless_network_list) => wireless_network_list,
            Err(err) => {
                // Convert the error into a gRPC Status and return it.
                return Err(Status::from_error(err.into()));
            }
        };
        //add wifi list to scan_results
        for wifi in wireless_network_list {
            let scan_result = ScanResult {
                mac: wifi.mac,
                frequency: wifi.frequency,
                signal: wifi.signal as i32,
                flags: wifi.flags,
                name: wifi.name,
            };
            scan_results.results.push(scan_result);
        }

        Ok(Response::new(scan_results))
    }

    async fn connect_wireless_network(
        &self,
        request: Request<WifiConnectRequest>,
    ) -> Result<Response<WifiConnectResponse>, Status> {
        let mut wifi_connect_response = WifiConnectResponse::default();
        let request_data = request.into_inner();

        self.handle_response(
            self.connect_to_wifi(&request_data.ssid, &request_data.psk)
                .await,
            &mut wifi_connect_response,
            NETWORK_CONNECT_SUCCESS_MESSAGE,
            NETWORK_CONNECT_FAILURE_MESSAGE,
        );

        Ok(Response::new(wifi_connect_response))
    }

    async fn disconnect_wireless_network(
        &self,
        request: Request<RemoveNetworkRequest>,
    ) -> Result<Response<RemoveNetworkResponse>, Status> {
        let mut remove_network_response = RemoveNetworkResponse::default();
        let request_data = request.into_inner();

        self.handle_response(
            self.remove_wifi_network(request_data.network_id as usize)
                .await,
            &mut remove_network_response,
            NETWORK_REMOVAL_SUCCESS_MESSAGE,
            NETWORK_REMOVAL_FAILURE_MESSAGE,
        );

        Ok(Response::new(remove_network_response))
    }

    async fn scan_known_wireless_network(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<NetworkResults>, Status> {
        // Implement your async get_known_wifi logic here
        let mut scan_results = NetworkResults::default();
        log::info!("Starting Known Wifi List Function");

        //get wifi list from mecha_edge_sdk
        let wireless_network_list =
            match WirelessNetworkControl::get_known_wireless_networks().await {
                Ok(wireless_network_list) => wireless_network_list,
                Err(err) => {
                    // Convert the error into a gRPC Status and return it.
                    return Err(Status::from_error(err.into()));
                }
            };
        //add wifi list to scan_results
        for wifi in wireless_network_list {
            let scan_result = NetworkResult {
                network_id: wifi.network_id as i32,
                flags: wifi.flags,
                ssid: wifi.ssid,
            };
            scan_results.results.push(scan_result);
        }

        Ok(Response::new(scan_results))
    }

    async fn get_wifi_status(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<WifiStatusResponse>, Status> {
        // Implement your logic to check Wi-Fi status here
        let wifi_on = WirelessNetworkControl::wireless_network_status().await; // This should return true if Wi-Fi is on, false otherwise.

        let wifi_status_response = WifiStatusResponse { wifi_on };

        Ok(Response::new(wifi_status_response))
    }

    async fn get_current_network(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ScanResult>, Status> {
        // Implement your logic to get current Wi-Fi network here
        let wifi_service = WirelessNetworkControl::new();
        let current_network = match wifi_service.current_wireless_network().await {
            Ok(current_network) => current_network,
            Err(err) => {
                // Convert the error into a gRPC Status and return it.
                return Err(Status::from_error(err.into()));
            }
        };

        let scan_result = ScanResult {
            mac: current_network.mac,
            frequency: current_network.frequency,
            signal: current_network.signal as i32,
            flags: current_network.flags,
            name: current_network.name,
        };

        Ok(Response::new(scan_result))
    }
}
