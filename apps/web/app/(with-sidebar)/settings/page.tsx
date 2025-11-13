"use client";

import { Button } from "@/components/atoms/Button/Button";
import {
	PasswordDialog,
	PasswordDialogPortal,
	PasswordDialogTrigger,
} from "@/components/organisms/PasswordDialog/PasswordDialog";
import { useSudo } from "@/hooks/auth";
import { useCurrentUser } from "@/hooks/user";
import { useEffect, useState } from "react";

export default function Settings() {
	let { sudo, mutateSudo } = useSudo();
	let { user, mutateUser } = useCurrentUser();

	const [open, setOpen] = useState(false);

	return (
		<div>
			<h1>Settings</h1>
			<h2>Account Settings</h2>
			{user.isLoading && <p>Loading user data...</p>}
			<PasswordDialog modal={true}>
				<PasswordDialogTrigger>
					<Button onClick={() => setOpen(true)}>
						Enter Sudo Mode
					</Button>
				</PasswordDialogTrigger>
				<PasswordDialogPortal
					onPassword={async (password) => {
						await mutateSudo({ password });
						setOpen(false);
					}}
				/>
			</PasswordDialog>
		</div>
	);
}
