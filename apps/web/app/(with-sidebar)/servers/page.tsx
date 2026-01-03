"use client";

import { Button } from "@/components/atoms/Button/Button";
import { DialogRoot } from "@/components/organisms/Dialog/Dialog";
import { ServerCreateDialogPortal } from "@/components/organisms/ServerCreateDialog/ServerCreateDialog";
import { ServerList } from "@/components/organisms/ServerList/ServerList";
import { useState } from "react";

export default function Servers() {
	const [createDialogOpen, setCreateDialogOpen] = useState(false);

	return (
		<div>
			<h1>Servers</h1>

			<Button onClick={() => setCreateDialogOpen(true)}>
				Create Server
			</Button>

			<DialogRoot
				modal={true}
				open={createDialogOpen}
				onOpenChange={(open) => setCreateDialogOpen(open)}
			>
				<ServerCreateDialogPortal />
			</DialogRoot>
			<ServerList />
		</div>
	);
}
