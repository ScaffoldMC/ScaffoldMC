import { cva } from "class-variance-authority";
import styles from "./indicator.module.css";

interface IndicatorProps {
	state: "success" | "working" | "error";
}

const indicatorStyles = cva(styles.base, {
	variants: {
		state: {
			success: styles.success,
			working: styles.working,
			error: styles.error,
		},
	},
});

export function Indicator(props: IndicatorProps) {
	const className = indicatorStyles({ state: props.state });
	return <div className={className}></div>;
}
