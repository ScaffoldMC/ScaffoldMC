import {
	Root,
	CheckboxIndicator,
	CheckboxProps,
} from "@radix-ui/react-checkbox";

import styles from "./Checkbox.module.css";
import { Check } from "lucide-react";

export function Checkbox({ ...props }: CheckboxProps) {
	return (
		<Root className={styles.checkboxRoot} {...props}>
			<CheckboxIndicator className={styles.checkboxIndicator}>
				<Check />
			</CheckboxIndicator>
		</Root>
	);
}
