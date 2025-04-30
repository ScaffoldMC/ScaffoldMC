import { Button } from "@/components/atoms/buttons/button";
import React from "react";
import styles from "./serverlist.module.css";
import Link from "next/link";
import { OctagonX, FolderSync } from "lucide-react";
import { Indicator } from "@/components/atoms/indicator/indicator";

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
		<div className={styles.item}>
			<Link href="/dashboard/1">Server 1</Link>

			<div className={styles.cluster}>
				<Indicator state="success" />
				<p>8/16 Active</p>
			</div>

			<div className={styles.cluster}>
				<Button size="icon" level="destructive">
					<OctagonX size={18} />
				</Button>
				<Button size="icon" level="secondary">
					<FolderSync size={18} />
				</Button>
			</div>
		</div>
	);
}

ServerList.Item = ServerListItem;
