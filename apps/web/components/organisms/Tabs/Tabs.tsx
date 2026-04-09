import * as TabsPrimitive from "@radix-ui/react-tabs";
import { cn } from "@/lib/util";

export function Tabs({
	children,
	className,
	...props
}: TabsPrimitive.TabsProps) {
	return (
		<TabsPrimitive.Root
			className={cn("flex w-full flex-col items-center", className)}
			{...props}
		>
			{children}
		</TabsPrimitive.Root>
	);
}

export function TabsList({
	children,
	className,
	...props
}: TabsPrimitive.TabsListProps) {
	return (
		<TabsPrimitive.List
			className={cn(
				"flex w-full items-center justify-between gap-2 p-1 bg-surface rounded-lg border border-border-static",
				className,
			)}
			{...props}
		>
			{children}
		</TabsPrimitive.List>
	);
}

export function TabsTrigger({
	children,
	...props
}: TabsPrimitive.TabsTriggerProps) {
	return (
		<TabsPrimitive.Trigger
			className={cn(
				"h-8 flex grow items-center justify-center rounded-md bg-transparent px-4 py-2",
				"transition-[background-color,border-color,color] duration-100 ease-in-out",
				"hover:cursor-pointer hover:bg-surface-overlay",
				"data-[state=active]:bg-accent-lightest dark:data-[state=active]:bg-accent-darkest",
			)}
			{...props}
		>
			{children}
		</TabsPrimitive.Trigger>
	);
}

export function TabsContent({
	children,
	...props
}: TabsPrimitive.TabsContentProps) {
	return (
		<TabsPrimitive.Content {...props} className="w-full">
			{children}
		</TabsPrimitive.Content>
	);
}
