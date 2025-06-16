"use client";

import { ButtonHTMLAttributes } from "react";
import { cva } from "class-variance-authority";
import styles from "./Button.module.css";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
	size?: "icon" | "default";
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
	},
	defaultVariants: {
		level: "secondary",
	},
});

export function Button({ level, ...props }: ButtonProps) {
	const className = buttonStyles({ level });
	return <button className={className} {...props}></button>;
}
