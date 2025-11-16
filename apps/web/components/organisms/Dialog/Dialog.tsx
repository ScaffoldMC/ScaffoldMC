"use client";

import * as DialogPrimitive from "@radix-ui/react-dialog";
import styles from "./Dialog.module.css";

export function DialogRoot({
	children,
	...props
}: DialogPrimitive.DialogProps) {
	return <DialogPrimitive.Root {...props}>{children}</DialogPrimitive.Root>;
}

export function DialogTrigger({ children }: { children?: React.ReactNode }) {
	return (
		<DialogPrimitive.Trigger asChild>{children}</DialogPrimitive.Trigger>
	);
}

export function DialogPortal({
	children,
	...props
}: DialogPrimitive.DialogPortalProps) {
	return (
		<DialogPrimitive.Portal {...props}>{children}</DialogPrimitive.Portal>
	);
}

export function DialogOverlay({
	...props
}: DialogPrimitive.DialogOverlayProps) {
	return <DialogPrimitive.Overlay className={styles.overlay} {...props} />;
}

export function DialogContent({
	children,
	...props
}: DialogPrimitive.DialogContentProps) {
	return (
		<DialogPrimitive.Content className={styles.content} {...props}>
			{children}
		</DialogPrimitive.Content>
	);
}
