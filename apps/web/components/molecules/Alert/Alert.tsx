import React from "react";
import styles from "./Alert.module.css";
import { cva } from "class-variance-authority";

interface AlertProps extends React.HTMLAttributes<HTMLDivElement> {
	type: "info" | "warning" | "error";
}

export function Alert({ children, type }: AlertProps) {
	const className = cva(styles.alert, {
		variants: {
			type: {
				info: styles.info,
				warning: styles.warning,
				error: styles.error,
			},
		},
	})({ type });

	return <div className={className}>{children}</div>;
}
