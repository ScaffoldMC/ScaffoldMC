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
				"flex w-full items-center justify-between gap-2 p-1",
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
				"h-8 flex grow items-center justify-center border-b-2 border-transparent bg-transparent px-4 py-2 text-text-secondary",
				"transition-[background-color,border-color,color] duration-100 ease-in-out hover:cursor-pointer",
				"hover:bg-surface-overlay hover:text-text-primary",
				"data-[state=active]:text-accent-base data-[state=active]:rounded-none data-[state=active]:border-accent-base",
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
	return <TabsPrimitive.Content {...props}>{children}</TabsPrimitive.Content>;
}
