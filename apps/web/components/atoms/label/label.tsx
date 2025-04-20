import styles from "./label.module.css";

export function Label({
	...props
}: React.LabelHTMLAttributes<HTMLLabelElement>) {
	return <label className={styles.base} {...props} />;
}
