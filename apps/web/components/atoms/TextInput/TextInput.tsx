import styles from "./TextInput.module.css";
import { cva } from "class-variance-authority";

const inputVariants = cva(styles.base, {
	variants: {
		invalid: {
			true: styles.invalid,
			false: "",
		},
	},
	defaultVariants: {
		invalid: false,
	},
});

export type TextInputProps = Omit<
	React.InputHTMLAttributes<HTMLInputElement>,
	"type"
> & {
	type?: "text" | "email" | "password" | "search" | "tel" | "url";
	invalid?: boolean;
};

export function TextInput({ type, invalid, ...props }: TextInputProps) {
	return (
		<input type={type} className={inputVariants({ invalid })} {...props} />
	);
}
