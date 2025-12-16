"use client";

import { Button } from "@/components/atoms/Button/Button";
import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import {
	DialogRoot,
	DialogTrigger,
} from "@/components/organisms/Dialog/Dialog";
import { PasswordDialogPortal } from "@/components/organisms/PasswordDialog/PasswordDialog";
import { useSudo } from "@/hooks/auth";
import { useCurrentUser } from "@/hooks/user";
import { Unlock, Lock, Edit, LockIcon } from "lucide-react";
import { useEffect, useState } from "react";
import styles from "./page.module.css";
import { Alert } from "@/components/molecules/Alert/Alert";
import { EditableTextInput } from "@/components/molecules/EditableTextInput/EditableTextInput";
import { PasswordResetPortal } from "@/components/organisms/PasswordResetDialog/PasswordResetDialog";

export default function Settings() {
	let { sudo, mutateSudo } = useSudo();
	let { user, mutateUser } = useCurrentUser();

	const [passwordEntryOpen, setPasswordEntryOpen] = useState(false);
	const [passwordResetOpen, setPasswordResetOpen] = useState(false);

	return (
		<div className={styles.layout}>
			<h1>Settings</h1>
			{!sudo.data && (
				<Alert type="warning">
					<div className={styles.alertContent}>
						<LockIcon size={18} />
						<b>Limited Access</b>
						<Button
							level="ghost"
							onClick={() => setPasswordEntryOpen(true)}
						>
							Unlock
						</Button>
					</div>
				</Alert>
			)}

			<DialogRoot
				open={passwordEntryOpen}
				modal={true}
				onOpenChange={(open) => setPasswordEntryOpen(open)}
			>
				<PasswordDialogPortal
					onPassword={async (password) => {
						await mutateSudo({ password });
						setPasswordEntryOpen(false);
					}}
				/>
			</DialogRoot>

			<DialogRoot
				open={passwordResetOpen}
				modal={true}
				onOpenChange={(open) => setPasswordResetOpen(open)}
			>
				<PasswordResetPortal
					onSubmit={async (oldPassword, newPassword) => {
						await mutateUser({
							password: oldPassword,
							new_password: newPassword,
						});
						setPasswordResetOpen(false);
					}}
				/>
			</DialogRoot>

			<h2>Account Settings</h2>
			<div className={styles.field}>
				<Label htmlFor="username">Username</Label>
				<div className={styles.textInput}>
					<EditableTextInput
						editable={sudo.data}
						value={user.data?.username || ""}
						onChange={async (value) => {
							await mutateUser({ username: value });
						}}
					/>
				</div>
			</div>
			<div className={styles.field}>
				<Label htmlFor="name">Name</Label>
				<div className={styles.textInput}>
					<EditableTextInput
						editable={sudo.data}
						value={user.data?.fullname || ""}
						onChange={async (value) => {
							await mutateUser({ fullname: value });
						}}
					/>
				</div>
			</div>
			<div className={styles.field}>
				<Label htmlFor="editpassword">Password</Label>
				<Button
					id="editpassword"
					disabled={!sudo.data}
					onClick={() => setPasswordResetOpen(true)}
				>
					Change Password
				</Button>
			</div>
		</div>
	);
}
