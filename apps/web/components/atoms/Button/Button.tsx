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
		"h-9 w-fit gap-2 px-2 border-0 rounded-sm",
		"transition-[background-color] duration-100 ease-in-out",
		"text-base text-black dark:text-white",
		"[&_svg]:inline-block disabled:cursor-not-allowed disabled:opacity-50",
	),
	{
		variants: {
			level: {
				primary:
					"bg-primary text-white enabled:hover:bg-[hsl(from_var(--color-primary)_h_s_calc(l-10))]",
				secondary: cn(
					"bg-gray-100 enabled:hover:bg-gray-200",
					"dark:bg-gray-900 dark:enabled:hover:bg-gray-800",
				),
				destructive: cn(
					"bg-red-200  text-red-900 hover:enabled:bg-red-300",
					"dark:bg-red-900 dark:text-red-300 dark:hover:enabled:bg-red-800",
				),
				ghost: cn(
					"bg-transparent enabled:hover:bg-[hsl(from_var(--color-secondary)_h_s_calc(l-10))]",
					"dark:enabled:hover:bg-[hsl(from_var(--color-secondary)_h_s_calc(l+10))]",
				),
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
		<button className={buttonStyles({ level, size, className })} {...props}>
			{children}
		</button>
	);
}
