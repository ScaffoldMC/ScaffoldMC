"use client";

import { Button } from "@/components/atoms/Button/Button";
import {
	DialogRoot,
	DialogTrigger,
} from "@/components/organisms/Dialog/Dialog";
import { PasswordDialogPortal } from "@/components/organisms/PasswordDialog/PasswordDialog";
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
			<DialogRoot modal={true}>
				<DialogTrigger>
					<Button onClick={() => setOpen(true)}>
						Enter Sudo Mode
					</Button>
				</DialogTrigger>
				<PasswordDialogPortal
					onPassword={async (password) => {
						await mutateSudo({ password });
						setOpen(false);
					}}
				/>
			</DialogRoot>
		</div>
	);
}
