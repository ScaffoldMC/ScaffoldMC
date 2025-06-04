import * as CheckboxPrimitive from "@radix-ui/react-checkbox";

import styles from "./checkbox.module.css";
import { Check } from "lucide-react";

export function Checkbox({
	children,
	...props
}: CheckboxPrimitive.CheckboxProps) {
	return (
		<CheckboxPrimitive.Root className={styles.checkboxRoot} {...props}>
			<CheckboxPrimitive.CheckboxIndicator
				className={styles.checkboxIndicator}
			>
				<Check />
			</CheckboxPrimitive.CheckboxIndicator>
		</CheckboxPrimitive.Root>
	);
}
