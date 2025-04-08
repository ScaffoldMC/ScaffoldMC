import styles from "./input.module.css";

export function Input({
	...props
}: React.InputHTMLAttributes<HTMLInputElement>) {
	return <input className={styles.base} {...props} />;
}
