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
				<Sidebar.Header>
					<h1 className={styles.logo}>App Name</h1>
					<p className={styles.tagline}>Some other info, probably.</p>
				</Sidebar.Header>
				<Sidebar.Links>
					<p> :3 </p>
				</Sidebar.Links>
				<Sidebar.Bottom>
					<User />
				</Sidebar.Bottom>
			</Sidebar>
			<div className={styles.content}>{children}</div>
		</div>
	);
}
