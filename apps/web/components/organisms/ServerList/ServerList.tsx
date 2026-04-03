"use client";

import React, { useEffect } from "react";
import Link from "next/link";
import { IndicatorState } from "@/components/atoms/Indicator/Indicator";
import { List, ListItem } from "@/components/organisms/List/List";
import { useQuery } from "@tanstack/react-query";
import api from "@/lib/axios";
import Image from "next/image";
import { cn, gameString } from "@/lib/util";
import { EthernetPort, Play, Square } from "lucide-react";
import { Button } from "@/components/atoms/Button/Button";

// TODO: Make brief & detailed view of server list

export function ServerList() {
	const serverIds = useQuery({
		queryKey: ["servers"],
		queryFn: () => api.get("/servers").then((res) => res.data),
		retry: false,
	});

	return (
		<List
			names={{
				singular: "Server",
				plural: "Servers",
			}}
		>
			{serverIds.data?.map((uuid: string) => (
				<ServerListItem key={uuid} uuid={uuid} />
			))}
		</List>
	);
}

function ServerListItem({ uuid }: { uuid: string }) {
	const serverInfo = useQuery({
		queryKey: ["server", uuid],
		queryFn: () => api.get(`/servers/${uuid}`).then((res) => res.data),
		retry: false,
	});

	let indicatorState: IndicatorState = "error";

	switch (serverInfo.data?.state) {
		case "Running":
			indicatorState = "success";
			break;
		case "Stopped":
		default:
			indicatorState = "error";
			break;
	}

	useEffect(() => {
		console.log(serverInfo.data);
	}, [serverInfo.data]);

	return (
		<ListItem>
			<Link
				href={`/servers/${uuid}`}
				className="font-medium text-text-primary no-underline"
			>
				<div
					className={cn(
						"flex flex-row items-center justify-between p-2",
						"transition-[background-color] duration-100 ease-in-out hover:bg-surface-raised",
					)}
				>
					<div className="flex w-fit flex-row items-center gap-2">
						<Image
							src="/images/server-default.png"
							alt="Server image"
							width={48}
							height={48}
							className="rounded-lg"
						/>

						<div>
							<p>{serverInfo.data?.name || "Server name"}</p>
							<p className="text-text-secondary text-xs">
								{serverInfo.data &&
									gameString(serverInfo.data.game)}
							</p>
						</div>
					</div>

					<div className="flex w-fit flex-row items-center gap-3">
						{serverInfo.data?.state === "Running" && (
							<div className="flex flex-row items-center gap-1 text-text-secondary">
								<EthernetPort size={14} />
								<p className="text-sm">25565</p>
							</div>
						)}

						{serverInfo.data?.state === "Running" ? (
							<Button level="secondary">
								<Square size={18} /> Stop
							</Button>
						) : (
							<Button level="secondary">
								<Play size={18} /> Start
							</Button>
						)}
					</div>
				</div>
			</Link>
		</ListItem>
	);
}
