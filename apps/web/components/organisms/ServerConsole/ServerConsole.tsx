import { Button } from "@/components/atoms/Button/Button";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { useServer } from "@/hooks/servers";
import { cn } from "@/lib/util";
import { ArrowDown, SendHorizonal } from "lucide-react";
import { SubmitEventHandler, useEffect, useRef, useState } from "react";
import { ConsoleLine } from "../../../../backend/bindings/ConsoleLine";

interface ServerConsoleProps {
	serverId: string;
}

export function ServerConsole({ serverId }: ServerConsoleProps) {
	const [consoleData, setConsoleData] = useState<ConsoleLine[]>([]);
	const { server, isRunning } = useServer(serverId);

	const handleCommandSubmit: SubmitEventHandler<HTMLFormElement> = (
		event,
	) => {
		event.preventDefault();

		const command = event.currentTarget.command.value;
		console.log(command);
	};

	if (!server.data) return null;
	if (server.isLoading) return <div>Loading...</div>;

	useEffect(() => {
		const base = process.env.NEXT_PUBLIC_API_BASE_URL?.endsWith("/")
			? process.env.NEXT_PUBLIC_API_BASE_URL
			: `${process.env.NEXT_PUBLIC_API_BASE_URL}/`;

		const endpoint = new URL(`servers/${serverId}/console`, base).href;

		let eventSource: EventSource | null = null;
		let isCancelled = false;

		const connect = () => {
			if (isCancelled) return;

			eventSource = new EventSource(endpoint, { withCredentials: true });

			eventSource.onopen = () => {
				console.log("Server SSE connection established");
			};

			eventSource.addEventListener("console", (event) => {
				console.log("Server SSE console event:", event.data.toString());
				setConsoleData((prev) => [
					...prev,
					...(JSON.parse(event.data) as ConsoleLine[]),
				]);
			});

			eventSource.onerror = (error) => {
				console.error("Server SSE error:", error);
				if (eventSource?.readyState === EventSource.CLOSED) {
					console.warn(
						"Connection closed, attempting reconnect in 3s...",
					);
					eventSource.close();
					if (!isCancelled) {
						setTimeout(connect, 3000);
					}
				}
			};
		};

		connect();

		return () => {
			isCancelled = true;
			eventSource?.close();
			console.log("Connection cleaned up");
		};
	}, [serverId, setConsoleData]);

	const consoleTextRef = useRef(null);
	const [hasUserScrolled, setHasUserScrolled] = useState(false);

	useEffect(() => {
		if (!consoleTextRef.current) return;

		const scrollHeight = consoleTextRef.current.scrollHeight;
		const clientHeight = consoleTextRef.current.clientHeight;

		if (!hasUserScrolled) {
			consoleTextRef.current.scrollTop = scrollHeight - clientHeight;
		}
	}, [consoleTextRef, consoleData, hasUserScrolled]);

	const scrollHandler = () => {
		const scrollPosition = consoleTextRef.current.scrollTop;
		const scrollHeight = consoleTextRef.current.scrollHeight;
		const clientHeight = consoleTextRef.current.clientHeight;

		if (scrollPosition < scrollHeight - clientHeight - 10) {
			setHasUserScrolled(true);
		} else {
			setHasUserScrolled(false);
		}
	};

	return (
		<div
			className={cn(
				"bg-surface p-2 h-100 w-full rounded-md border border-border-static",
				"flex flex-col gap-2",
			)}
		>
			<div className="relative flex-1 min-h-0">
				<div
					ref={consoleTextRef}
					className="flex flex-col gap-0.5 overflow-scroll max-h-full"
					onScroll={scrollHandler}
				>
					{consoleData.map((line, index) => (
						<span className="text-sm font-mono" key={index}>
							{line.line}
						</span>
					))}
					<Button
						hidden={!hasUserScrolled}
						className="absolute bottom-0 right-0"
						onClick={() => setHasUserScrolled(false)}
					>
						<ArrowDown size={18} />
					</Button>
				</div>
			</div>
			<form
				className="flex flex-row flex-stretch gap-2 w-full"
				onSubmit={handleCommandSubmit}
			>
				<TextInput
					disabled={!isRunning}
					name="command"
					placeholder="Enter command"
					className="flex-1"
				/>
				<Button level="primary" type="submit" disabled={!isRunning}>
					<SendHorizonal size={18} />
				</Button>
			</form>
		</div>
	);
}
