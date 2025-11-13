import * as DialogPrimitive from "@radix-ui/react-dialog";

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
