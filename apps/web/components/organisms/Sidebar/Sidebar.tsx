"use client";

import Link, { LinkProps } from "next/link";
import { useEffect, useState } from "react";
import { usePathname } from "next/navigation";
import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

export function Sidebar({ children }: { children: React.ReactNode }) {
	return (
		<div className="flex h-screen w-62.5 flex-col bg-foreground shadow-sm">
			{children}
		</div>
	);
}

export function SidebarHeader({ children }: { children: React.ReactNode }) {
	return <div className="sticky top-0 flex flex-col p-3">{children}</div>;
}

export function SidebarContent({ children }: { children?: React.ReactNode }) {
	return (
		<div className="flex w-full grow flex-col gap-1 overflow-y-auto p-2">
			{children}
		</div>
	);
}

export function SidebarFooter({ children }: { children: React.ReactNode }) {
	return <div className="sticky bottom-0 mt-auto p-3">{children}</div>;
}

const activeClassName = cva(
	cn(
		"flex items-center justify-start gap-2 rounded-md p-2 no-underline",
		"transition-[background-color] duration-100 ease-in-out",
		"hover:bg-hover",
	),
	{
		variants: {
			active: {
				true: "bg-primary-background text-primary hover:bg-primary-background",
				false: "",
			},
		},
		defaultVariants: {
			active: false,
		},
	},
);

export function SidebarLink({
	children,
	...props
}: {
	children: React.ReactNode;
} & LinkProps) {
	const pathname = usePathname();
	const [isActive, setIsActive] = useState(false);
	const className = activeClassName({ active: isActive });

	useEffect(() => {
		setIsActive(pathname === props.href.toString());
	}, [pathname, props.href]);

	return (
		<Link className={className} {...props}>
			{children}
		</Link>
	);
}

export function SublinkGroup({
	children,
	baseUrl,
}: {
	children: React.ReactNode;
	baseUrl: string;
}) {
	const pathname = usePathname();
	const [isActive, setIsActive] = useState(false);

	useEffect(() => {
		setIsActive(pathname.startsWith(baseUrl));
	}, [pathname, baseUrl]);

	return isActive ? (
		<div className="flex flex-row gap-0.5 pl-4 [&_a]:text-sm">
			<div className="flex flex-col gap-0.5">{children}</div>
		</div>
	) : null;
}
