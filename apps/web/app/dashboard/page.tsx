import styles from "./page.module.css";
import { Server } from "lucide-react";
import { ServerList } from "@/components/organisms/serverlist/serverlist";

export default function Dashboard() {
	return (
		<div className={styles.dashboard}>
			<h2>Servers</h2>
			<ServerList />
		</div>
	);
}
