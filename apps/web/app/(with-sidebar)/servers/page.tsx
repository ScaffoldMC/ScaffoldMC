"use client";

import { Button } from "@/components/atoms/Button/Button";
import { DialogRoot } from "@/components/organisms/Dialog/Dialog";
import { ServerCreateDialog } from "@/components/organisms/ServerCreateDialog/ServerCreateDialog";
import { ServerList } from "@/components/organisms/ServerList/ServerList";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default function Servers() {
	const router = useRouter();
	const [createDialogOpen, setCreateDialogOpen] = useState(false);

	const handleServerCreate = () => {
		setCreateDialogOpen(false);
	};

	return (
		<div className="flex flex-col gap-2">
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
		</div>
	);
}
