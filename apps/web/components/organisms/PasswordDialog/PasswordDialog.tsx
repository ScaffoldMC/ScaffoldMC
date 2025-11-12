import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Button } from "@/components/atoms/Button/Button";
import { useState } from "react";
import styles from "./PasswordDialog.module.css";

interface PasswordDialogProps {
	onPassword?: (password: string) => Promise<void>;
	onCancel?: () => void;
}

export function PasswordDialog({ onPassword, onCancel }: PasswordDialogProps) {
	const [isError, setIsError] = useState(false);

	const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		const formData = new FormData(event.currentTarget);
		const password = formData.get("password") as string;

		onPassword(password).catch(() => {
			setIsError(true);
		});
	};

	return (
		<dialog className={styles.dialog}>
			<h3>Authenticate to continue</h3>
			<form className={styles.form} onSubmit={handleSubmit}>
				<div className={styles.field}>
					<Label htmlFor="password">Password</Label>
					<TextInput
						type="password"
						id="password"
						name="password"
						invalid={isError}
						required
					/>
				</div>
				<Button type="submit" level="primary">
					Continue
				</Button>
				<Button type="button" level="secondary" onClick={onCancel}>
					Cancel
				</Button>
			</form>
		</dialog>
	);
}
