"use client";

import { Button } from "@/components/atoms/Button/Button";
import { Label } from "@/components/atoms/Label/Label";
import { DialogRoot } from "@/components/organisms/Dialog/Dialog";
import { PasswordDialogPortal } from "@/components/organisms/PasswordDialog/PasswordDialog";
import { useSudo } from "@/hooks/auth";
import { useCurrentUser } from "@/hooks/user";
import { useState } from "react";
import { EditableTextInput } from "@/components/molecules/EditableTextInput/EditableTextInput";
import { PasswordResetPortal } from "@/components/organisms/PasswordResetDialog/PasswordResetDialog";
import PageLayout from "@/components/atoms/PageLayout/PageLayout";
import FormField from "@/components/atoms/FormField/FormField";
import { LockIcon, UnlockIcon } from "lucide-react";

export default function Settings() {
	let { sudo, mutateSudo } = useSudo();
	let { user, mutateUser } = useCurrentUser();

	const [passwordEntryOpen, setPasswordEntryOpen] = useState(false);
	const [passwordResetOpen, setPasswordResetOpen] = useState(false);

	return (
		<PageLayout>
			<div className="flex flex-row justify-between items-center">
				<h1>Settings</h1>
				<div className="flex flex-row gap-4 items-center text-sm">
					{!sudo.data && (
						<span className="text-text-secondary">
							Unlock to make changes
						</span>
					)}

					<Button
						disabled={sudo.data}
						level="secondary"
						onClick={() => setPasswordEntryOpen(true)}
					>
						{sudo.data ? (
							<UnlockIcon size={18} />
						) : (
							<LockIcon size={18} />
						)}
					</Button>
				</div>
			</div>

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

			<h2>Your Account</h2>
			<div className="flex flex-col gap-4 p-4 bg-surface rounded-md border border-border-static">
				<FormField>
					<Label htmlFor="username">Username</Label>
					<EditableTextInput
						editable={sudo.data}
						value={user.data?.username || ""}
						onChange={async (value) => {
							await mutateUser({ username: value });
						}}
					/>
				</FormField>
				<FormField>
					<Label htmlFor="name">Name</Label>
					<EditableTextInput
						editable={sudo.data}
						value={user.data?.fullname || ""}
						onChange={async (value) => {
							await mutateUser({ fullname: value });
						}}
					/>
				</FormField>
				<FormField>
					<Label htmlFor="editpassword">Password</Label>
					<Button
						id="editpassword"
						onClick={() => setPasswordResetOpen(true)}
					>
						Change Password
					</Button>
				</FormField>
			</div>
		</PageLayout>
	);
}
