import {
	Sidebar,
	SidebarContent,
	SidebarHeader,
	SidebarFooter,
	SidebarLink,
} from "@/components/organisms/sidebar/sidebar";
import styles from "./layout.module.css";
import { User } from "@/components/molecules/user/user";
import { Home, Server, Settings } from "lucide-react";

export default function DashboardLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<div className={styles.dashboard}>
			<Sidebar>
				<SidebarHeader>
					<h1 className={styles.logo}>App Name</h1>
					<p className={styles.tagline}>Some other info, probably.</p>
				</SidebarHeader>
				<SidebarContent>
					<SidebarLink href="/dashboard">
						<Home size={16} className={styles.icon} />
						Dashboard
					</SidebarLink>
					<SidebarLink href="/dashboard/servers">
						<Server size={16} className={styles.icon} />
						Servers
					</SidebarLink>
					<SidebarLink href="/dashboard/settings">
						<Settings size={16} className={styles.icon} />
						Settings
					</SidebarLink>
				</SidebarContent>
				<SidebarFooter>
					<User />
				</SidebarFooter>
			</Sidebar>
			<div className={styles.content}>{children}</div>
		</div>
	);
}
