"use client";

import Link, { LinkProps } from "next/link";
import { useEffect, useState } from "react";
import { usePathname } from "next/navigation";
import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";
import { User } from "@/components/molecules/User/User";
import Image from "next/image";

export function Navbar({ children }: { children: React.ReactNode }) {
	return (
		<nav className="flex h-14 flex-row border-b border-border-static bg-surface sticky top-0">
			<div className="flex items-center w-16 mx-3 bg-orange">
				<Image
					src="/images/logo.svg"
					alt="ScaffoldMC Logo"
					width={64}
					height={64}
				/>
			</div>
			<div className="flex w-full gap-2 overflow-y-auto py-2">
				{children}
			</div>
			<div className="flex h-full w-72 items-center gap-4 p-2 border-l border-border-static">
				<User />
			</div>
		</nav>
	);
}

const buttonClassName = cva(
	cn(
		"flex flex-row items-center justify-start gap-1.5 p-2 no-underline rounded-md",
		"transition-[background-color,border-color,color] duration-100 ease-in-out",
	),
	{
		variants: {
			active: {
				true: cn(
					"bg-accent-lightest text-primary hover:bg-accent-lightest",
					"dark:bg-accent-darkest dark:text-accent-lightest dark:hover:bg-accent-darkest",
				),
				false: "hover:text-text-primary text-text-secondary border-transparent",
			},
		},
		defaultVariants: {
			active: false,
		},
	},
);

export function NavbarLink({
	children,
	...props
}: {
	children: React.ReactNode;
} & LinkProps) {
	const pathname = usePathname();
	const [isActive, setIsActive] = useState(false);
	const className = buttonClassName({ active: isActive });

	useEffect(() => {
		setIsActive(pathname === props.href.toString());
	}, [pathname, props.href]);

	return (
		<Link className={className} {...props}>
			{children}
		</Link>
	);
}
