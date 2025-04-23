import { ButtonHTMLAttributes } from "react";
import { cva } from "class-variance-authority";
import styles from "./button.module.css";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
	size?: "icon" | "default";
	level?: "primary" | "secondary" | "destructive";
}

const buttonStyles = cva(styles.base, {
	variants: {
		level: {
			primary: styles.primary,
			secondary: styles.secondary,
			destructive: styles.destructive,
		},
		size: {
			default: styles.defaultSize,
			icon: styles.iconSize,
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
