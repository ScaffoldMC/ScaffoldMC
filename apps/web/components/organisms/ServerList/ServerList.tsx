"use client";

import Link from "next/link";
import { List, ListItem } from "@/components/organisms/List/List";
import { useQuery } from "@tanstack/react-query";
import api from "@/lib/axios";
import Image from "next/image";
import { cn, gameString } from "@/lib/util";
import { EthernetPort } from "lucide-react";
import { ServerStartStopButton } from "../ServerStartStopButton/ServerStartStopButton";

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
			emptyView={
				<span className="flex flex-col h-36 items-center justify-center text-sm text-text-secondary">
					There&#39;s nothing here yet! Create a server to get started
				</span>
			}
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

	// TODO: Get port for server

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
							<p>
								{serverInfo.data?.config.name || "Server name"}
							</p>
							<p className="text-text-secondary text-xs">
								{serverInfo.data &&
									gameString(serverInfo.data.config.game)}
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

						<ServerStartStopButton serverId={uuid} />
					</div>
				</div>
			</Link>
		</ListItem>
	);
}
