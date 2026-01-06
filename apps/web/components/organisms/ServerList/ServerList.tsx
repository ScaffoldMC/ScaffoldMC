"use client";

import React from "react";
import styles from "./ServerList.module.css";
import Link from "next/link";
import {
	Indicator,
	IndicatorState,
} from "@/components/atoms/Indicator/Indicator";
import {
	Avatar,
	AvatarFallback,
	AvatarImage,
} from "@/components/atoms/Avatar/Avatar";
import { List, ListItem } from "@/components/organisms/List/List";
import { useQuery } from "@tanstack/react-query";
import api from "@/lib/axios";

// TODO: Hook up to backend logic
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

	return (
		<ListItem>
			<Link href={`/servers/${uuid}`} className={styles.link}>
				<div className={styles.item}>
					<div className={styles.statusCluster}>
						<Avatar size={28} shape="square-small">
							<AvatarFallback>?</AvatarFallback>
							<AvatarImage src="/images/server-default.png" />
						</Avatar>

						<p>{serverInfo.data?.name || "Server name"}</p>
					</div>

					<div className={styles.statusCluster}>
						<Indicator state={indicatorState} />
						<p>{serverInfo.data?.state || "Unknown"}</p>
					</div>
				</div>
			</Link>
		</ListItem>
	);
}
