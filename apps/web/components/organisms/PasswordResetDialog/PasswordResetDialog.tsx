"use client";

import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Button } from "@/components/atoms/Button/Button";
import { useState } from "react";
import styles from "./PasswordResetDialog.module.css";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogContent, DialogOverlay, DialogPortal } from "../Dialog/Dialog";

export interface PasswordResetPortalProps
	extends DialogPrimitive.DialogPortalProps {
	onSubmit?: (oldPassword: string, newPassword: string) => Promise<void>;
}

export function PasswordResetPortal({ onSubmit }: PasswordResetPortalProps) {
	const [isError, setIsError] = useState(false);

	const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		const formData = new FormData(event.currentTarget);
		const oldPassword = formData.get("password") as string;
		const newPassword = formData.get("newPassword") as string;

		onSubmit(oldPassword, newPassword).catch(() => {
			setIsError(true);
		});
	};

	return (
		<DialogPortal>
			<DialogOverlay />
			<DialogContent>
				<DialogPrimitive.Title>Reset Password</DialogPrimitive.Title>
				<DialogPrimitive.Description>
					Enter your old and new password.
				</DialogPrimitive.Description>
				<form className={styles.form} onSubmit={handleSubmit}>
					<div className={styles.field}>
						<Label htmlFor="password">Old Password</Label>
						<TextInput
							type="password"
							id="password"
							name="password"
							invalid={isError}
							required
						/>
					</div>
					<div className={styles.field}>
						<Label htmlFor="newPassword">New Password</Label>
						<TextInput
							type="password"
							id="newPassword"
							name="newPassword"
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
