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
			console.error("Failed to start/stop server:", error);
		}

		setLoading(false);
	};

	useEffect(() => {
		if (isStarting) {
			setLoading(true);
		}
	}, [isStarting]);

	if (server.isError) {
		console.error(`Server ${serverId} could not be found: ${server.error}`);
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
