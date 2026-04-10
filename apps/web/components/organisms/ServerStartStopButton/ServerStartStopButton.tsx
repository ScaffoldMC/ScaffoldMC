import { Button } from "@/components/atoms/Button/Button";
import { useServer } from "@/hooks/servers";
import { LoaderCircle, Play, Square } from "lucide-react";
import { MouseEventHandler, useState } from "react";

export function ServerStartStopButton({ serverId }: { serverId: string }) {
	const { server, isRunning, startServer, stopServer } = useServer(serverId);
	const [loading, setLoading] = useState(false);

	if (server.isError) {
		console.error(`Server ${serverId} could not be found: ${server.error}`);
		return null;
	}

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

	return (
		<Button level="secondary" onClick={handleStartStop} disabled={loading}>
			{loading && <LoaderCircle size={18} className="animate-spin" />}
			{!loading &&
				(isRunning ? <Square size={18} /> : <Play size={18} />)}
			{isRunning ? "Stop" : "Start"}
		</Button>
	);
}
