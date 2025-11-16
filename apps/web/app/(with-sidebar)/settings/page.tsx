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
import { Unlock, Lock, Edit } from "lucide-react";
import { useEffect, useState } from "react";
import styles from "./page.module.css";

export default function Settings() {
	let { sudo, mutateSudo } = useSudo();
	let { user, mutateUser } = useCurrentUser();

	const [open, setOpen] = useState(false);

	return (
		<div className={styles.layout}>
			<h1>Settings</h1>
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

			<h2>Account Settings</h2>
			<div className={styles.field}>
				<Label htmlFor="username">Username</Label>
				<TextInput
					id="username"
					placeholder="Username"
					disabled={!sudo.data}
					value={user.data?.username || ""}
				/>
			</div>
			<div className={styles.field}>
				<Label htmlFor="name">Name</Label>
				<TextInput
					id="name"
					placeholder="Name"
					disabled={!sudo.data}
					value={user.data?.fullname || ""}
				/>
			</div>
			<div className={styles.field}>
				<Label htmlFor="editpassword">Password</Label>
				<Button id="editpassword" disabled={!sudo.data} size="icon">
					<Edit size={18} />
				</Button>
			</div>
		</div>
	);
}
