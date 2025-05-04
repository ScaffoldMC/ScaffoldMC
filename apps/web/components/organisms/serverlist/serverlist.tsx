import { Button } from "@/components/atoms/buttons/button";
import React from "react";
import styles from "./serverlist.module.css";
import Link from "next/link";
import { OctagonX, FolderSync } from "lucide-react";
import { Indicator } from "@/components/atoms/indicator/indicator";
import {
	Avatar,
	AvatarFallback,
	AvatarImage,
} from "@/components/atoms/avatar/avatar";

// TODO: Hook up to backend logic

export function ServerList({ children }: { children?: React.ReactNode }) {
	return (
		<div className={styles.root}>
			<div className={styles.header}>
				<b>n servers</b>
			</div>
			{children}
		</div>
	);
}

function ServerListItem() {
	return (
		<Link href="/dashboard/1" className={styles.link}>
			<div className={styles.item}>
				<div className={styles.statusCluster}>
					<Avatar size={28} shape="square">
						<AvatarFallback>?</AvatarFallback>
						<AvatarImage src="images/server-default.png" />
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
	);
}

ServerList.Item = ServerListItem;
