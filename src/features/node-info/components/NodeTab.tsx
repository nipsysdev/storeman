import { useStore } from "@nanostores/react";
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
	Typography,
} from "@nipsysdev/lsd-react";
import {
	$connectionStatus,
	ConnectionStatus,
} from "../../connection/connectionStore";
import { $nodePeerId, $nodeVersion } from "../nodeStore";

export default function NodeTab() {
	const connectionStatus = useStore($connectionStatus);
	const nodeVersion = useStore($nodeVersion);
	const nodePeerId = useStore($nodePeerId);

	const nodeInfo = (
		<>
			<div className="flex flex-col">
				<Typography variant="subtitle1">Version</Typography>
				<Typography color="secondary">{nodeVersion}</Typography>
			</div>
			<div className="flex flex-col col-span-3">
				<Typography variant="subtitle1">Peer Id</Typography>
				<Typography color="secondary" style={{ overflowWrap: "anywhere" }}>
					{nodePeerId}
				</Typography>
			</div>
		</>
	);

	return (
		<Card className="size-full border-0!">
			<CardHeader className="border-x">
				<CardTitle>Node Information</CardTitle>
				<CardDescription>See information about the active node</CardDescription>
			</CardHeader>
			<CardContent className="space-y-4 flex-auto grid grid-cols-4">
				{connectionStatus === ConnectionStatus.Connected ? (
					nodeInfo
				) : (
					<div className="col-span-full text-center">
						<Typography variant="subtitle1" color="secondary">
							Not connected
						</Typography>
					</div>
				)}
			</CardContent>
		</Card>
	);
}
