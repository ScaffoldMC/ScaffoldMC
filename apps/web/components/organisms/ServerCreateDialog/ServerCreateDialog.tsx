import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogContent, DialogOverlay, DialogPortal } from "../Dialog/Dialog";
import { VersionSelector } from "../VersionSelector/VersionSelector";
import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import styles from "./ServerCreateDialog.module.css";
import { Button } from "@/components/atoms/Button/Button";

export function ServerCreateDialogPortal() {
	return (
		<DialogPortal>
			<DialogOverlay />
			<DialogContent>
				<DialogPrimitive.Title>Create Server</DialogPrimitive.Title>
				<VisuallyHidden>
					<DialogPrimitive.Description>
						Select the software and configure your new server.
					</DialogPrimitive.Description>
				</VisuallyHidden>
				<form className={styles.form}>
					<div className={styles.field}>
						<Label>Server Software</Label>
						<VersionSelector />
					</div>
					<div className={styles.field}>
						<Label>Server Name</Label>
						<TextInput />
					</div>
					<Button type="submit" level="primary">
						Create
					</Button>
					<DialogPrimitive.Close asChild>
						<Button type="button" level="secondary">
							Cancel
						</Button>
					</DialogPrimitive.Close>
				</form>
			</DialogContent>
		</DialogPortal>
	);
}
