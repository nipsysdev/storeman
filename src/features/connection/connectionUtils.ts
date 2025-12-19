import { ConnectionStatus } from "./connectionTypes";

export function getConnectionStatusText(status: ConnectionStatus) {
    switch (status) {
                case ConnectionStatus.Connected:
                    return "Connected";
                case ConnectionStatus.Connecting:
                    return "Connecting...";
                case ConnectionStatus.Error:
                    return "Error";
                case ConnectionStatus.Initialized:
                case ConnectionStatus.Disconnected:
                    return "Disconnected";
                default:
                    return status;
            }
}

export function getConnectionStatusDescription(status: ConnectionStatus, connectionError?: string | null) {
		switch (status) {
			case ConnectionStatus.Connecting:
				return "Connecting to Storage network...";
			case ConnectionStatus.Connected:
				return "Connected to Storage successfully";
			case ConnectionStatus.Error:
				return (
					connectionError || "An error occurred while connecting to Storage"
				);
			case ConnectionStatus.Disconnected:
			case ConnectionStatus.Initialized:
				return "Disconnected from Storage network";
			default:
				return "";
		}
	};