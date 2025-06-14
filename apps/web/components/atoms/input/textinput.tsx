import styles from "./textinput.module.css";

export type TextInputProps = Omit<
	React.InputHTMLAttributes<HTMLInputElement>,
	"type"
> & {
	type?: "text" | "email" | "password" | "search" | "tel" | "url";
};

export function TextInput({ type, ...props }: TextInputProps) {
	return <input type={type} className={styles.base} {...props} />;
}
