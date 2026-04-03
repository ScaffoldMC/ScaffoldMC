import PageLayout from "@/components/atoms/PageLayout/PageLayout";
import styles from "./page.module.css";
import { ServerList } from "@/components/organisms/ServerList/ServerList";

export default function Dashboard() {
	return (
		<PageLayout>
			<h1>Hello John Doe</h1>
			<div className={styles.dashboardGrid}>
				<ServerList />
			</div>
		</PageLayout>
	);
}
