import { Button } from "@/components/atoms/Button/Button";
import { useServer } from "@/hooks/servers";
import {
	DialogContent,
	DialogOverlay,
	DialogPortal,
	DialogRoot,
} from "../Dialog/Dialog";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { useState } from "react";

export function ServerDangerActions({ serverId }: { serverId: string }) {
	const { server } = useServer(serverId);
	const [confirmOpen, setConfirmOpen] = useState(false);

	// TODO: Add delete from backend
	// TODO: Add countdown to confirm button

	if (!server.data) {
		return null;
	}

	return (
		<div className="flex flex-col gap-4 p-4 bg-surface rounded-md border border-border-static">
			<DialogRoot
				open={confirmOpen}
				onOpenChange={(v) => setConfirmOpen(v)}
			>
				<DialogPortal>
					<DialogOverlay />
					<DialogContent className="gap-4 items-start p-6 w-screen max-w-100">
						<DialogPrimitive.Title>
							Delete server?
						</DialogPrimitive.Title>
						<DialogPrimitive.Description>
							Are you sure you want to delete this server? This
							action cannot be undone.
						</DialogPrimitive.Description>
						<div className="self-end flex flex-row gap-2">
							<Button type="button" level="destructive">
								Delete Server
							</Button>
							<DialogPrimitive.Close asChild>
								<Button type="button" level="secondary">
									Cancel
								</Button>
							</DialogPrimitive.Close>
						</div>
					</DialogContent>
				</DialogPortal>
			</DialogRoot>

			<div className="w-full flex justify-between items-center">
				<span className="text-xl">Delete Server</span>
				<Button
					level="destructive"
					onClick={() => setConfirmOpen(true)}
				>
					Delete Server
				</Button>
			</div>
		</div>
	);
}
