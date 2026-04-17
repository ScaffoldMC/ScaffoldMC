import { Game } from "@/lib/servertypes";
import { VersionSelector } from "../VersionSelector/VersionSelector";
import { useCallback, useEffect, useState } from "react";
import { AlertOctagon, Edit, X } from "lucide-react";
import { Button } from "@/components/atoms/Button/Button";
import { gameString } from "@/lib/util";
import {
	DialogContent,
	DialogOverlay,
	DialogPortal,
	DialogRoot,
} from "../Dialog/Dialog";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Alert } from "@/components/molecules/Alert/Alert";

export interface VersionChangerProps {
	value?: Game;
	onChange: (game: Game | null) => void;
}

export function VersionChanger({ value, onChange }: VersionChangerProps) {
	const [selectorOpen, setSelectorOpen] = useState(false);
	const [warnings, setWarnings] = useState<string[]>([]);
	const [newValue, setNewValue] = useState<Game | null>(null);

	const onGame = useCallback(
		(game?: Game) => {
			if (!game || game === null) return;

			setWarnings([]);

			// Generate warnings for user if the configuration is changing significantly
			if (game.type !== value.type) {
				setWarnings((w) => [
					...w,
					"A different game type is selected. Server files may be incompatible or overwritten.",
				]);
			} else if (game.loader != value?.loader) {
				setWarnings((w) => [
					...w,
					"A different loader is selected. Mods or plugins may be incompatible.",
				]);
			}

			setNewValue(game);
		},
		[value],
	);

	const onConfirm = () => {
		if (!newValue) return;

		onChange(newValue);
		setSelectorOpen(false);
	};

	// Clear warnings on close to prevent stale warnings
	useEffect(() => {
		if (!selectorOpen) {
			setWarnings([]);
		}
	}, [selectorOpen]);

	return (
		<>
			<DialogRoot
				open={selectorOpen}
				onOpenChange={(v) => setSelectorOpen(v)}
			>
				<DialogPortal>
					<DialogOverlay />
					<DialogContent className="gap-4 items-start p-6 w-screen max-w-200">
						<div className="flex justify-between w-full">
							<DialogPrimitive.Title>
								Change game version
							</DialogPrimitive.Title>
							<DialogPrimitive.Close asChild>
								<Button type="button" level="ghost">
									<X size={18} />
								</Button>
							</DialogPrimitive.Close>
						</div>

						<DialogPrimitive.Description>
							Select a new game version for your server to use.
						</DialogPrimitive.Description>

						{warnings.map((warning, i) => (
							<Alert key={i} type="warning">
								<AlertOctagon size={18} />
								{warning}
							</Alert>
						))}

						<VersionSelector onGame={onGame} />

						<div className="self-end flex flex-row gap-2">
							<DialogPrimitive.Close asChild>
								<Button type="button" level="secondary">
									Cancel
								</Button>
							</DialogPrimitive.Close>
							<Button
								level="primary"
								disabled={!newValue}
								onClick={onConfirm}
							>
								Confirm
							</Button>
						</div>
					</DialogContent>
				</DialogPortal>
			</DialogRoot>
			<div
				hidden={selectorOpen}
				className="flex flex-row gap-4 items-center justify-start"
			>
				<span className="text-text-secondary">
					{value ? gameString(value) : "No game set (how?)"}
				</span>

				<Button level="secondary" onClick={() => setSelectorOpen(true)}>
					<Edit size={18} />
				</Button>
			</div>
		</>
	);
}
