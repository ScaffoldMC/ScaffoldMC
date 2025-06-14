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
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export default async function DashboardLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	// Redirect if the user doesn't have a refresh token.
	// This isn't an actual auth check, but a cleaner way to redirect without a
	// flash of web content.

	const refresh_token = (await cookies()).get("refresh_token")?.value;

	if (!refresh_token) {
		redirect("/login");
	}

	return (
		<div className={styles.dashboard}>
			<Sidebar>
				<SidebarHeader>
					<h1 className={styles.logo}>App Name</h1>
					<p className={styles.tagline}>Some other info, probably.</p>
				</SidebarHeader>
				<SidebarContent>
					<SidebarLink href="/home">
						<Home size={18} className={styles.icon} />
						Dashboard
					</SidebarLink>
					<SidebarLink href="/servers">
						<Server size={18} className={styles.icon} />
						Servers
					</SidebarLink>
					<SidebarLink href="/settings">
						<Settings size={18} className={styles.icon} />
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
