import styles from "./sidebar.module.css";

export function Sidebar({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebar}>{children}</div>;
}
