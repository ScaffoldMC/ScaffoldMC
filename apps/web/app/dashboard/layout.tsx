import { Sidebar } from "@/components/organisms/sidebar/sidebar";
import styles from "./layout.module.css";

export default function DashboardLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<div className={styles.dashboard}>
			<Sidebar>
				<p>placeholder</p>
			</Sidebar>
			<div className={styles.content}>{children}</div>
		</div>
	);
}
