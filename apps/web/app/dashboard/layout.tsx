import { Sidebar } from "@/components/organisms/sidebar/sidebar";
import styles from "./layout.module.css";
import { User } from "@/components/molecules/user/user";

export default function DashboardLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<div className={styles.dashboard}>
			<Sidebar>
				<User />
			</Sidebar>
			<div className={styles.content}>{children}</div>
		</div>
	);
}
