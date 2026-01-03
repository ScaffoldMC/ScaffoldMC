import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogContent, DialogOverlay, DialogPortal } from "../Dialog/Dialog";
import { VersionSelector } from "../VersionSelector/VersionSelector";
import { VisuallyHidden } from "@radix-ui/react-visually-hidden";

export function ServerCreateDialogPortal() {
	return (
		<DialogPortal>
			<DialogOverlay />
			<DialogContent>
				<DialogPrimitive.Title>Create Server</DialogPrimitive.Title>
				<VisuallyHidden>
					<DialogPrimitive.Description>
						Select the edition and version for your new server.
					</DialogPrimitive.Description>
				</VisuallyHidden>
				<VersionSelector />
			</DialogContent>
		</DialogPortal>
	);
}
