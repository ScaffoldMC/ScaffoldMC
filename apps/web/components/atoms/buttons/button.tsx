import { ButtonHTMLAttributes } from "react";
import { cva } from "class-variance-authority";
import styles from "./button.module.css";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
	level?: "primary" | "secondary" | "destructive";
}

const buttonStyles = cva(styles.base, {
	variants: {
		level: {
			primary: styles.primary,
			secondary: styles.secondary,
			destructive: styles.destructive,
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
