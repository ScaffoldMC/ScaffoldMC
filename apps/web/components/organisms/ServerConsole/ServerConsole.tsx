import { Button } from "@/components/atoms/Button/Button";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { useServer } from "@/hooks/servers";
import { cn } from "@/lib/util";
import { ArrowDown, LoaderCircle, SendHorizonal } from "lucide-react";
import { SubmitEventHandler, useEffect, useRef, useState } from "react";
import { ConsoleLine } from "../../../../backend/bindings/ConsoleLine";

interface ServerConsoleProps {
	serverId: string;
}

/// Helper to apply a class based on a regex
function formatLine(
	line: string,
	patterns: { regex: RegExp; className: string }[],
) {
	const segments = [];
	let pos = 0;

	while (pos < line.length) {
		let earliest = null;

		for (const pattern of patterns) {
			const match = pattern.regex.exec(line.slice(pos));
			if (match) {
				const absIndex = pos + match.index;
				if (earliest === null || absIndex < earliest.absIndex) {
					earliest = { match, pattern, absIndex };
				}
			}
		}

		if (!earliest) {
			segments.push(<span key={pos}>{line.slice(pos)}</span>);
			break;
		}

		const { match, pattern, absIndex } = earliest;

		if (absIndex > pos) {
			segments.push(<span key={pos}>{line.slice(pos, absIndex)}</span>);
		}

		segments.push(
			<span key={absIndex} className={pattern.className}>
				{match[0]}
			</span>,
		);

		pos = absIndex + match[0].length;
	}

	return segments;
}

function FormattedConsoleLine({ line }: { line: ConsoleLine }) {
	if (line.stream == "Stderr") {
		return (
			<span className="text-sm font-mono text-red-700 dark:text-red-300">
				{line.line}
			</span>
		);
	}

	const patterns = [
		{
			regex: /^\[\d+:\d+:\d+\]/g,
			className: "text-gray-600 dark:text-gray-400",
		},
		{
			regex: /\[[\w\s]+[ /]INFO\]:/,
			className: "text-blue-600 dark:text-blue-400",
		},
		{
			regex: /\[[\w\s]+[ /]WARN\]:/,
			className: "text-orange-600 dark:text-orange-400",
		},
	];

	return (
		<span className="text-sm font-mono">
			{formatLine(line.line, patterns)}
		</span>
	);
}

export function ServerConsole({ serverId }: ServerConsoleProps) {
	const { server, isRunning, sendCommand } = useServer(serverId);
	const [consoleData, setConsoleData] = useState<ConsoleLine[]>([]);
	const [commandLoading, setCommandLoading] = useState(false);
	const [hasUserScrolled, setHasUserScrolled] = useState(false);
	const consoleTextRef = useRef(null);
	const commandBoxRef = useRef<HTMLInputElement>(null);

	const handleCommandSubmit: SubmitEventHandler<HTMLFormElement> = (
		event,
	) => {
		event.preventDefault();

		const command = event.currentTarget.command.value;
		setCommandLoading(true);

		// TODO: Error feedback
		sendCommand(command)
			.then(() => {
				setCommandLoading(false);
			})
			.finally(() => {
				commandBoxRef.current.value = "";
			});
	};

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
				console.log("Console SSE connection established");
				setConsoleData([]);
			};

			eventSource.addEventListener("console", (event) => {
				setConsoleData((prev) => [
					...prev,
					...(JSON.parse(event.data) as ConsoleLine[]),
				]);
			});

			eventSource.onerror = () => {
				if (eventSource?.readyState === EventSource.CLOSED) {
					eventSource.close();
					if (!isCancelled) {
						setTimeout(connect, 1000);
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

	useEffect(() => {
		if (!consoleTextRef.current) return;

		const scrollHeight = consoleTextRef.current.scrollHeight;
		const clientHeight = consoleTextRef.current.clientHeight;

		if (!hasUserScrolled) {
			consoleTextRef.current.scrollTop = scrollHeight - clientHeight;
		}
	}, [consoleTextRef, consoleData, hasUserScrolled]);

	if (!server.data) return null;
	if (server.isLoading) return <div>Loading...</div>;

	return (
		<div
			className={cn(
				"bg-surface p-2 h-120 w-full rounded-md border border-border-static",
				"flex flex-col gap-2",
			)}
		>
			<div className="relative flex-1 min-h-0">
				<div
					ref={consoleTextRef}
					className="flex flex-col gap-0.5 overflow-y-scroll max-h-full"
					onScroll={scrollHandler}
				>
					{consoleData.map((line, index) => (
						<FormattedConsoleLine key={index} line={line} />
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
					ref={commandBoxRef}
					disabled={!isRunning}
					name="command"
					placeholder="Enter command"
					className="flex-1"
				/>
				<Button
					level="primary"
					type="submit"
					disabled={!isRunning || commandLoading}
				>
					{commandLoading ? (
						<div className="animate-spin w-full h-full flex items-center justify-center">
							<LoaderCircle size={18} />
						</div>
					) : (
						<SendHorizonal size={18} />
					)}
				</Button>
			</form>
		</div>
	);
}
