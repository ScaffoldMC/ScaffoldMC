"use client";

import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogContent, DialogOverlay, DialogPortal } from "../Dialog/Dialog";
import { Button } from "@/components/atoms/Button/Button";
import { LoaderCircle, X } from "lucide-react";
import { FormEvent, SubmitEventHandler, useState } from "react";
import { Game } from "@/lib/servertypes";
import { useServers } from "@/hooks/servers";
import { Label } from "@/components/atoms/Label/Label";
import { VersionSelector } from "../VersionSelector/VersionSelector";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { VisuallyHidden } from "@radix-ui/react-visually-hidden";

interface ServerCreateDialogProps extends DialogPrimitive.DialogPortalProps {
	onServerCreate: () => void;
}

export function ServerCreateDialog({
	onServerCreate,
}: ServerCreateDialogProps) {
	const [game, setGame] = useState<Game | null>(null);
	const { mutateServers } = useServers();
	const [isLoading, setIsLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);

	const handleSubmit: SubmitEventHandler<HTMLFormElement> = (event) => {
		event.preventDefault();
		const formData = new FormData(event.currentTarget);
		const name = formData.get("name") as string;

		if (!game || !name) return;

		setIsLoading(true);
		mutateServers({ name, game }).then((res) => {
			setIsLoading(false);
			if (res.status === 201) {
				onServerCreate();
			}
		});
	};

	const handleGame = (selectedGame?: Game) => {
		setGame(selectedGame);
	};

	return (
		<DialogPortal>
			<DialogOverlay />
			<DialogContent className="gap-2 items-start p-6 w-screen max-w-200">
				<div className="flex justify-between w-full">
					<DialogPrimitive.Title>Create Server</DialogPrimitive.Title>
					<DialogPrimitive.Close asChild>
						<Button type="button" level="ghost">
							<X size={18} />
						</Button>
					</DialogPrimitive.Close>
				</div>
				<VisuallyHidden>
					<DialogPrimitive.Description>
						Create a new server from scratch.
					</DialogPrimitive.Description>
				</VisuallyHidden>
				<form
					onSubmit={handleSubmit}
					className="flex flex-col gap-2 *:flex *:flex-col *:gap-1 w-full"
				>
					<div>
						<Label>Name</Label>
						<TextInput name="name" />
					</div>
					<div>
						<Label>Software</Label>
						<VersionSelector onGame={handleGame} />
					</div>
					<div className="items-end">
						<Button
							className="w-30"
							type="submit"
							level="primary"
							disabled={isLoading}
						>
							{isLoading ? (
								<LoaderCircle className="animate-spin" />
							) : (
								"Create"
							)}
						</Button>
					</div>
				</form>
			</DialogContent>
		</DialogPortal>
	);
}
