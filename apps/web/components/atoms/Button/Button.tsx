"use client";

import { ButtonHTMLAttributes } from "react";
import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
	size?: "default" | "variable";
	level?: "primary" | "secondary" | "destructive" | "ghost";
}

const buttonStyles = cva(
	cn(
		"inline-flex items-center justify-center font-semibold cursor-pointer",
		"h-9 w-fit gap-2 rounded-md border px-2",
		"transition-[background-color,border-color,color] duration-100 ease-in-out",
		"text-base",
		"[&_svg]:inline-block disabled:cursor-not-allowed disabled:opacity-50",
	),
	{
		variants: {
			level: {
				primary: cn(
					"border-transparent bg-accent-base text-surface-raised",
					"enabled:hover:bg-accent-dark",
				),
				secondary: cn(
					"border-border-static bg-surface-overlay text-text-primary",
					"enabled:hover:bg-surface",
				),
				destructive: cn(
					"border-red-200 bg-red-100 text-red-900",
					"dark:border-red-900 dark:bg-red-950 dark:text-red-50",
					"enabled:hover:bg-red-200 dark:enabled:hover:bg-red-800",
				),
				ghost: "border-transparent bg-transparent text-text-primary enabled:hover:bg-surface-overlay",
			},
			size: {
				default: "h-9 min-w-9",
				variable: "h-fit px-2 py-0",
			},
		},
		defaultVariants: {
			level: "secondary",
			size: "default",
		},
	},
);

export function Button({
	level,
	size,
	className,
	children,
	...props
}: ButtonProps) {
	return (
		<button
			className={cn(buttonStyles({ level, size }), className)}
			{...props}
		>
			{children}
		</button>
	);
}
