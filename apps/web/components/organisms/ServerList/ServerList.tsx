import React from "react";
import styles from "./ServerList.module.css";
import Link from "next/link";
import { Indicator } from "@/components/atoms/Indicator/Indicator";
import {
	Avatar,
	AvatarFallback,
	AvatarImage,
} from "@/components/atoms/Avatar/Avatar";
import { List, ListItem } from "@/components/organisms/List/List";

// TODO: Hook up to backend logic
// TODO: Make brief & detailed view of server list

export function ServerList() {
	return (
		<List
			names={{
				singular: "Server",
				plural: "Servers",
			}}
		>
			<ServerListItem />
			<ServerListItem />
			<ServerListItem />
			<ServerListItem />
		</List>
	);
}

function ServerListItem() {
	return (
		<ListItem>
			<Link href="/servers/1" className={styles.link}>
				<div className={styles.item}>
					<div className={styles.statusCluster}>
						<Avatar size={28} shape="square-small">
							<AvatarFallback>?</AvatarFallback>
							<AvatarImage src="/images/server-default.png" />
						</Avatar>

						<p>Server name</p>
					</div>

					<div className={styles.statusCluster}>
						<p>1/10 Online</p>
					</div>

					<div className={styles.statusCluster}>
						<Indicator state="success" />
						<p>Active</p>
					</div>
				</div>
			</Link>
		</ListItem>
	);
}
