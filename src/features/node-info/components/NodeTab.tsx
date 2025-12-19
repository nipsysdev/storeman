import { useStore } from "@nanostores/react";
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
	Separator,
	Typography,
} from "@nipsysdev/lsd-react";
import { $nodeInfo } from "../nodeStore";

export default function NodeTab() {
	const nodeInfo = useStore($nodeInfo);

	return (
		<Card className="size-full border-0!">
			<CardHeader className="border-x">
				<CardTitle>Node Information</CardTitle>
				<CardDescription>
					See general and debug information about the node
				</CardDescription>
			</CardHeader>
			<CardContent className="flex flex-col gap-y-6">
				{nodeInfo ? (
					<div className="space-y-4 grid grid-cols-2 sm:grid-cols-6 lg:grid-cols-8">
						<div className="flex flex-col col-span-full sm:col-span-4">
							<Typography variant="subtitle1">Peer Id</Typography>
							<Typography color="secondary" className="wrap-anywhere">
								{nodeInfo?.peer_id}
							</Typography>
						</div>
						<div className="flex flex-col sm:col-span-2">
							<Typography variant="subtitle1">Node Version</Typography>
							<Typography color="secondary">{nodeInfo?.version}</Typography>
						</div>
						<div className="flex flex-col col-span-full sm:col-span-4">
							<Typography variant="subtitle1">Repository Path</Typography>
							<Typography color="secondary" className="wrap-anywhere">
								{nodeInfo?.repo_path}
							</Typography>
						</div>
						<div className="flex flex-col col-span-full sm:col-span-4">
							<Typography variant="subtitle1">Addresses</Typography>
							{nodeInfo?.debug_info?.addrs.map((address) => (
								<Typography
									color="secondary"
									className="wrap-anywhere"
									key={address}
								>
									{address}
								</Typography>
							)) ?? <Typography color="secondary">N/A</Typography>}
						</div>
						<div className="flex flex-col col-span-full sm:col-span-4">
							<Typography variant="subtitle1">Announce Addresses</Typography>
							{nodeInfo?.debug_info?.announceAddresses.map((address) => (
								<Typography
									color="secondary"
									className="wrap-anywhere"
									key={address}
								>
									{address}
								</Typography>
							)) ?? <Typography color="secondary">N/A</Typography>}
						</div>
					</div>
				) : (
					<div className="col-span-full text-center">
						<Typography variant="subtitle2" color="secondary">
							Node information unavailable
						</Typography>
					</div>
				)}
			</CardContent>
		</Card>
	);
}
