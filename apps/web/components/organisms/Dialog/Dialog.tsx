"use client";

import * as DialogPrimitive from "@radix-ui/react-dialog";
import { cn } from "@/lib/util";

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
	className,
	...props
}: DialogPrimitive.DialogOverlayProps) {
	return (
		<DialogPrimitive.Overlay
			className={cn(
				"fixed inset-0 z-1000 bg-black/50 backdrop-blur-sm",
				className,
			)}
			{...props}
		/>
	);
}

export function DialogContent({
	children,
	className,
	...props
}: DialogPrimitive.DialogContentProps) {
	return (
		<DialogPrimitive.Content
			className={cn(
				"fixed left-1/2 top-1/2 z-1001 flex w-100 -translate-x-1/2 -translate-y-1/2",
				"flex-col items-center justify-center gap-6 rounded-lg bg-foreground p-12 text-text shadow-sm",
				className,
			)}
			{...props}
		>
			{children}
		</DialogPrimitive.Content>
	);
}
