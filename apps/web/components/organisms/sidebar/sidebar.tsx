import Link, { LinkProps } from "next/link";
import styles from "./sidebar.module.css";

export function Sidebar({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebar}>{children}</div>;
}

export function Header({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebarHeader}>{children}</div>;
}

Sidebar.Header = Header;

export function Content({ children }: { children?: React.ReactNode }) {
	return <div className={styles.sidebarContent}>{children}</div>;
}

Sidebar.Content = Content;

export function Footer({ children }: { children: React.ReactNode }) {
	return <div className={styles.sidebarFooter}>{children}</div>;
}

Sidebar.Footer = Footer;

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

Sidebar.Link = SidebarLink;
