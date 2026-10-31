import { Button } from "@/components/atoms/Button/Button";
import { useServer } from "@/hooks/servers";
import { LoaderCircle, Play, Square } from "lucide-react";
import { MouseEventHandler, useEffect, useState } from "react";

export function ServerStartStopButton({ serverId }: { serverId: string }) {
	const { server, isRunning, isStarting, startServer, stopServer } =
		useServer(serverId);
	const [loading, setLoading] = useState(false);

	const handleStartStop: MouseEventHandler = async (event) => {
		event.stopPropagation();
		event.preventDefault();
		setLoading(true);

		try {
			if (isRunning) {
				await stopServer();
			} else {
				await startServer();
			}
		} catch (error) {
			// TODO: Swap out browser alert with a toast
			alert(
				`Failed to ${isRunning ? "start" : "stop"} server ${server.data?.config.name}: ${error.response.data}`,
			);
		}

		setLoading(false);
	};

	useEffect(() => {
		if (isStarting) {
			setLoading(true);
		}
	}, [isStarting]);

	if (server.isError) {
		return null;
	}

	return (
		<Button level="secondary" onClick={handleStartStop} disabled={loading}>
			{loading && <LoaderCircle size={18} className="animate-spin" />}
			{!loading &&
				(isRunning ? <Square size={18} /> : <Play size={18} />)}
			{isRunning ? "Stop" : "Start"}
		</Button>
	);
}
