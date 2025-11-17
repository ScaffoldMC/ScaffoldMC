"use client";

import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Button } from "@/components/atoms/Button/Button";
import { useState } from "react";
import styles from "./PasswordDialog.module.css";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogContent, DialogOverlay, DialogPortal } from "../Dialog/Dialog";

export interface PasswordDialogPortalProps
	extends DialogPrimitive.DialogPortalProps {
	onPassword?: (password: string) => Promise<void>;
}

export function PasswordDialogPortal({
	onPassword,
}: PasswordDialogPortalProps) {
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
		<DialogPortal>
			<DialogOverlay />
			<DialogContent>
				<DialogPrimitive.Title>
					Authenticate to continue
				</DialogPrimitive.Title>
				<DialogPrimitive.Description>
					Your password is required to change certain settings.
				</DialogPrimitive.Description>
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
					<DialogPrimitive.Close asChild>
						<Button type="button" level="secondary">
							Cancel
						</Button>
					</DialogPrimitive.Close>
				</form>
			</DialogContent>
		</DialogPortal>
	);
}
