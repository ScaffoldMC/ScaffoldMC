import styles from "./page.module.css";
import { Server } from "lucide-react";
import { ServerList } from "@/components/organisms/serverlist/serverlist";

export default function Dashboard() {
	return (
		<div className={styles.dashboard}>
			<h1>
				<Server size={24} /> Servers
			</h1>
			<ServerList>
				<ServerList.Item />
				<ServerList.Item />
				<ServerList.Item />
				<ServerList.Item />
			</ServerList>
		</div>
	);
}
