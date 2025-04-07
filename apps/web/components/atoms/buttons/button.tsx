import { ButtonHTMLAttributes, ComponentType } from "react";
import { cva } from "class-variance-authority";
import styles from "./button.module.css";

export enum ButtonPriority {
	Primary = "primary",
	Secondary = "secondary",
	Destructive = "destructive",
}

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
	label: string;
	icon?: ComponentType;
	level?: ButtonPriority;
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
		level: ButtonPriority.Secondary,
	},
});

export function Button({ label, level, ...props }: ButtonProps) {
	const className = buttonStyles({ level });
	return (
		<button className={className} {...props}>
			{label}
		</button>
	);
}
