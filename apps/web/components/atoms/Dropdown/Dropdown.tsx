import styles from "./Dropdown.module.css";

export function Dropdown({
	children,
	...props
}: React.SelectHTMLAttributes<HTMLSelectElement>) {
	return (
		<select className={styles.select} {...props}>
			{children}
		</select>
	);
}
