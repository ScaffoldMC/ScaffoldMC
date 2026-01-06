import styles from "./page.module.css";
import { Server } from "lucide-react";
import { ServerList } from "@/components/organisms/ServerList/ServerList";

export default function Dashboard() {
	return (
		<div className={styles.dashboard}>
			<h1>Hello John Doe</h1>
			<div className={styles.dashboardGrid}>
				<ServerList />
			</div>
		</div>
	);
}
