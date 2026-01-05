"use client";

import { ButtonHTMLAttributes } from "react";
import { cva } from "class-variance-authority";
import styles from "./Button.module.css";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
	size?: "default" | "variable";
	level?: "primary" | "secondary" | "destructive" | "ghost";
}

const buttonStyles = cva(styles.base, {
	variants: {
		level: {
			primary: styles.primary,
			secondary: styles.secondary,
			destructive: styles.destructive,
			ghost: styles.ghost,
		},
		size: {
			default: styles.defaultSize,
			variable: styles.variableSize,
		},
	},
	defaultVariants: {
		level: "secondary",
		size: "default",
	},
});

export function Button({ level, size, ...props }: ButtonProps) {
	const className = buttonStyles({ level, size });
	return <button className={className} {...props}></button>;
}
