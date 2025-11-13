"use client";

import { Button } from "@/components/atoms/Button/Button";
import {
	DialogRoot,
	DialogTrigger,
} from "@/components/organisms/Dialog/Dialog";
import { PasswordDialogPortal } from "@/components/organisms/PasswordDialog/PasswordDialog";
import { useSudo } from "@/hooks/auth";
import { useCurrentUser } from "@/hooks/user";
import { Unlock, Lock } from "lucide-react";
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
			{sudo.data && <p>Sudo mode is active.</p>}

			<DialogRoot
				open={open}
				modal={true}
				onOpenChange={(open) => setOpen(open)}
			>
				<DialogTrigger>
					<Button
						size="icon"
						onClick={() => setOpen(true)}
						disabled={sudo.data}
					>
						{sudo.data ? <Unlock size={18} /> : <Lock size={18} />}
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
