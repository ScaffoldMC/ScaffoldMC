"use client";

import { Button } from "@/components/atoms/Button/Button";
import { FSEntry } from "@/lib/servertypes";
import * as DialogPrimitive from "@radix-ui/react-dialog";

export function DeleteEntryDialog({
	entry,
	deleting,
	onCancel,
	onConfirm,
}: {
	entry: FSEntry | null;
	deleting: boolean;
	onCancel: () => void;
	onConfirm: () => void;
}) {
	return (
		<DialogPrimitive.Root
			modal={true}
			open={entry !== null}
			onOpenChange={(open) => {
				if (!open && !deleting) {
					onCancel();
				}
			}}
		>
			<DialogPrimitive.Portal>
				<DialogPrimitive.Overlay className="fixed inset-0 z-1000 bg-black/40" />
				<DialogPrimitive.Content className="fixed left-1/2 top-1/2 z-1001 flex w-100 -translate-x-1/2 -translate-y-1/2 flex-col items-start gap-4 rounded-lg border border-border-static bg-surface-raised p-6 text-text-primary">
					<DialogPrimitive.Title>
						Delete {entry?.type === "dir" ? "directory" : "file"}?
					</DialogPrimitive.Title>
					<DialogPrimitive.Description>
						Are you sure you want to delete{" "}
						<span className="font-medium">{entry?.name}</span>? This
						action cannot be undone.
					</DialogPrimitive.Description>
					<div className="flex w-full justify-end gap-2">
						<DialogPrimitive.Close asChild>
							<Button
								type="button"
								level="secondary"
								disabled={deleting}
							>
								Cancel
							</Button>
						</DialogPrimitive.Close>
						<Button
							type="button"
							level="destructive"
							disabled={deleting}
							onClick={onConfirm}
						>
							{deleting ? "Deleting..." : "Delete"}
						</Button>
					</div>
				</DialogPrimitive.Content>
			</DialogPrimitive.Portal>
		</DialogPrimitive.Root>
	);
}
