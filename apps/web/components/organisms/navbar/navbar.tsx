import React, { HTMLAttributes, ReactNode } from "react";
import styles from "./navbar.module.css";
import Link, { LinkProps } from "next/link";
import { Url } from "next/dist/shared/lib/router/router";

export function Navbar({ children }: { children: ReactNode }) {
	return <div className={styles.navRoot}>{children}</div>;
}

function NavbarLink({
	href,
	children,
	...props
}: LinkProps & { href: Url; children: ReactNode }) {
	return (
		<Link className={styles.navLink} href={href} {...props}>
			{children}
		</Link>
	);
}

Navbar.Link = NavbarLink;

function NavbarLogo({
	children,
	...props
}: {
	children: ReactNode;
	props?: HTMLAttributes<HTMLDivElement>;
}) {
	return (
		<div className={styles.navLogo} {...props}>
			{children}
		</div>
	);
}

Navbar.Logo = NavbarLogo;

function NavDivider() {
	return <div className={styles.navDivider} />;
}

Navbar.Divider = NavDivider;
