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

export default function Settings() {
	let { sudo, mutateSudo } = useSudo();
	let { user, mutateUser } = useCurrentUser();

	const [open, setOpen] = useState(false);

	return (
		<div className={styles.layout}>
			<h1>Settings</h1>
			{!sudo.data && (
				<Alert type="warning">
					<div className={styles.alertContent}>
						<LockIcon size={18} />
						<b>Limited Access</b>
						<Button level="ghost" onClick={() => setOpen(true)}>
							Unlock
						</Button>
					</div>
				</Alert>
			)}

			<DialogRoot
				open={open}
				modal={true}
				onOpenChange={(open) => setOpen(open)}
			>
				<PasswordDialogPortal
					onPassword={async (password) => {
						await mutateSudo({ password });
						setOpen(false);
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
				<Button id="editpassword" disabled={!sudo.data}>
					Change Password
				</Button>
			</div>
		</div>
	);
}
