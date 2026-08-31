"use client";

import * as ContextMenuPrimitive from "@radix-ui/react-context-menu";
import { cn } from "@/lib/util";

export function ContextMenuRoot({
	children,
	...props
}: ContextMenuPrimitive.ContextMenuProps) {
	return (
		<ContextMenuPrimitive.Root {...props}>
			{children}
		</ContextMenuPrimitive.Root>
	);
}

export const ContextMenu = ContextMenuRoot;

export function ContextMenuTrigger({
	children,
	...props
}: ContextMenuPrimitive.ContextMenuTriggerProps) {
	return (
		<ContextMenuPrimitive.Trigger {...props}>
			{children}
		</ContextMenuPrimitive.Trigger>
	);
}

export function ContextMenuPortal({
	children,
	...props
}: ContextMenuPrimitive.ContextMenuPortalProps) {
	return (
		<ContextMenuPrimitive.Portal {...props}>
			{children}
		</ContextMenuPrimitive.Portal>
	);
}

export function ContextMenuContent({
	children,
	className,
	...props
}: ContextMenuPrimitive.ContextMenuContentProps) {
	return (
		<ContextMenuPrimitive.Content
			className={cn(
				"z-50 min-w-48 overflow-hidden rounded-md border border-border-static bg-surface-raised p-1 text-sm text-text-primary shadow-lg",
				"animate-in fade-in-80",
				className,
			)}
			{...props}
		>
			{children}
		</ContextMenuPrimitive.Content>
	);
}

export function ContextMenuLabel({
	children,
	className,
	...props
}: ContextMenuPrimitive.ContextMenuLabelProps) {
	return (
		<ContextMenuPrimitive.Label
			className={cn(
				"px-2 py-1.5 text-xs font-semibold text-text-secondary",
				className,
			)}
			{...props}
		>
			{children}
		</ContextMenuPrimitive.Label>
	);
}

export function ContextMenuItem({
	children,
	className,
	...props
}: ContextMenuPrimitive.ContextMenuItemProps) {
	return (
		<ContextMenuPrimitive.Item
			className={cn(
				"relative flex cursor-pointer items-center rounded px-2 py-1.5 outline-none",
				"transition-[background-color,border-color,color] duration-100 ease-in-out",
				"hover:bg-surface-overlay hover:text-text-primary",
				"data-disabled:pointer-events-none data-disabled:opacity-50 data-highlighted:bg-surface-overlay data-highlighted:text-text-primary",
				className,
			)}
			{...props}
		>
			{children}
		</ContextMenuPrimitive.Item>
	);
}

export function ContextMenuGroup({
	children,
	...props
}: ContextMenuPrimitive.ContextMenuGroupProps) {
	return (
		<ContextMenuPrimitive.Group {...props}>
			{children}
		</ContextMenuPrimitive.Group>
	);
}

export function ContextMenuCheckboxItem({
	children,
	className,
	...props
}: ContextMenuPrimitive.ContextMenuCheckboxItemProps) {
	return (
		<ContextMenuPrimitive.CheckboxItem
			className={cn(
				"relative flex cursor-pointer select-none items-center rounded py-1.5 pl-8 pr-2 outline-none",
				"transition-[background-color,border-color,color] duration-100 ease-in-out",
				"hover:bg-surface-overlay hover:text-text-primary",
				"data-disabled:pointer-events-none data-disabled:opacity-50 data-highlighted:bg-surface-overlay data-highlighted:text-text-primary",
				className,
			)}
			{...props}
		>
			{children}
		</ContextMenuPrimitive.CheckboxItem>
	);
}

export function ContextMenuItemIndicator({
	children,
	className,
	...props
}: ContextMenuPrimitive.ContextMenuItemIndicatorProps) {
	return (
		<ContextMenuPrimitive.ItemIndicator
			className={cn(
				"absolute left-2 inline-flex items-center",
				className,
			)}
			{...props}
		>
			{children}
		</ContextMenuPrimitive.ItemIndicator>
	);
}

export function ContextMenuRadioGroup({
	children,
	...props
}: ContextMenuPrimitive.ContextMenuRadioGroupProps) {
	return (
		<ContextMenuPrimitive.RadioGroup {...props}>
			{children}
		</ContextMenuPrimitive.RadioGroup>
	);
}

export function ContextMenuRadioItem({
	children,
	className,
	...props
}: ContextMenuPrimitive.ContextMenuRadioItemProps) {
	return (
		<ContextMenuPrimitive.RadioItem
			className={cn(
				"relative flex cursor-pointer select-none items-center rounded py-1.5 pl-8 pr-2 outline-none",
				"transition-[background-color,border-color,color] duration-100 ease-in-out",
				"hover:bg-surface-overlay hover:text-text-primary",
				"data-disabled:pointer-events-none data-disabled:opacity-50 data-highlighted:bg-surface-overlay data-highlighted:text-text-primary",
				className,
			)}
			{...props}
		>
			{children}
		</ContextMenuPrimitive.RadioItem>
	);
}

export function ContextMenuSub({
	children,
	...props
}: ContextMenuPrimitive.ContextMenuSubProps) {
	return (
		<ContextMenuPrimitive.Sub {...props}>
			{children}
		</ContextMenuPrimitive.Sub>
	);
}

export function ContextMenuSubTrigger({
	children,
	className,
	...props
}: ContextMenuPrimitive.ContextMenuSubTriggerProps) {
	return (
		<ContextMenuPrimitive.SubTrigger
			className={cn(
				"flex cursor-pointer select-none items-center rounded px-2 py-1.5 outline-none",
				"transition-[background-color,border-color,color] duration-100 ease-in-out",
				"hover:bg-surface-overlay hover:text-text-primary",
				"data-highlighted:bg-surface-overlay data-highlighted:text-text-primary",
				className,
			)}
			{...props}
		>
			{children}
		</ContextMenuPrimitive.SubTrigger>
	);
}

export function ContextMenuSubContent({
	children,
	className,
	...props
}: ContextMenuPrimitive.ContextMenuSubContentProps) {
	return (
		<ContextMenuPrimitive.SubContent
			className={cn(
				"z-50 min-w-48 overflow-hidden rounded-md border border-border-static bg-surface-raised p-1 text-sm text-text-primary shadow-lg",
				className,
			)}
			{...props}
		>
			{children}
		</ContextMenuPrimitive.SubContent>
	);
}

export function ContextMenuSeparator({
	className,
	...props
}: ContextMenuPrimitive.ContextMenuSeparatorProps) {
	return (
		<ContextMenuPrimitive.Separator
			className={cn("my-1 h-px bg-border-static", className)}
			{...props}
		/>
	);
}
