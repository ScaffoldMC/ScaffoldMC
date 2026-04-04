"use client";

import { Button } from "@/components/atoms/Button/Button";
import PageLayout from "@/components/atoms/PageLayout/PageLayout";
import { DialogRoot } from "@/components/organisms/Dialog/Dialog";
import { ServerCreateDialog } from "@/components/organisms/ServerCreateDialog/ServerCreateDialog";
import { ServerList } from "@/components/organisms/ServerList/ServerList";
import { useState } from "react";

export default function Servers() {
	const [createDialogOpen, setCreateDialogOpen] = useState(false);

	const handleServerCreate = () => {
		setCreateDialogOpen(false);
	};

	return (
		<PageLayout>
			<div className="flex justify-between">
				<h1>Servers</h1>
				<Button onClick={() => setCreateDialogOpen(true)}>
					Create Server
				</Button>
			</div>

			<DialogRoot
				open={createDialogOpen}
				modal={true}
				onOpenChange={(open) => setCreateDialogOpen(open)}
			>
				<ServerCreateDialog onServerCreate={handleServerCreate} />
			</DialogRoot>

			<ServerList />
		</PageLayout>
	);
}
