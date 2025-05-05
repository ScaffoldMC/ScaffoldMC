import { Sidebar } from "@/components/organisms/sidebar/sidebar";
import styles from "./layout.module.css";
import { User } from "@/components/molecules/user/user";
import { Home, Settings } from "lucide-react";

export default function DashboardLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<div className={styles.dashboard}>
			<Sidebar>
				<Sidebar.Header>
					<h1 className={styles.logo}>App Name</h1>
					<p className={styles.tagline}>Some other info, probably.</p>
				</Sidebar.Header>
				<Sidebar.Content>
					<Sidebar.Link href="/dashboard">
						<Home size={16} className={styles.icon} />
						Dashboard
					</Sidebar.Link>
					<Sidebar.Link href="/dashboard/settings">
						<Settings size={16} className={styles.icon} />
						Settings
					</Sidebar.Link>
				</Sidebar.Content>
				<Sidebar.Footer>
					<User />
				</Sidebar.Footer>
			</Sidebar>
			<div className={styles.content}>{children}</div>
		</div>
	);
}
