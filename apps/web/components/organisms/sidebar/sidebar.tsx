"use client";

import Link, { LinkProps } from "next/link";
import styles from "./sidebar.module.css";
import { useEffect, useState } from "react";
import { usePathname } from "next/navigation";
import { cva } from "class-variance-authority";

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

const activeClassName = cva(styles.sidebarLink, {
	variants: {
		active: {
			true: styles.sidebarLinkActive,
			false: "",
		},
	},
	defaultVariants: {
		active: false,
	},
});

export function SidebarLink({
	children,
	...props
}: { children: React.ReactNode } & LinkProps) {
	const pathname = usePathname();
	const [isActive, setIsActive] = useState(false);
	const className = activeClassName({ active: isActive });

	useEffect(() => {
		setIsActive(() => pathname === props.href);
	}, [pathname, props.href]);

	return (
		<Link className={className} {...props}>
			{children}
		</Link>
	);
}
