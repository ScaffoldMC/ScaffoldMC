"use client";

import Link, { LinkProps } from "next/link";
import styles from "./sidebar.module.css";

export function Sidebar({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebar}>{children}</div>;
}

export function SidebarHeader({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebarHeader}>{children}</div>;
}

export function SidebarContent({ children }: { children?: React.ReactNode }) {
	return <div className={styles.sidebarContent}>{children}</div>;
}

export function SidebarFooter({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebarFooter}>{children}</div>;
}

export function SidebarLink({
	children,
	...props
}: { children: React.ReactNode } & LinkProps) {
	return (
		<Link className={styles.sidebarLink} {...props}>
			{children}
		</Link>
	);
}
