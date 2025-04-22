import styles from "./sidebar.module.css";

export function Sidebar({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebar}>{children}</div>;
}

export function Header({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebarHeader}>{children}</div>;
}

Sidebar.Header = Header;

export function Links({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebarLinks}>{children}</div>;
}

Sidebar.Links = Links;

export function Bottom({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebarBottom}>{children}</div>;
}

Sidebar.Bottom = Bottom;
