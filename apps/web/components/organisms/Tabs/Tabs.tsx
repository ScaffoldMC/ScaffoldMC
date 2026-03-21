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
				"flex w-full items-center justify-between gap-2 rounded-[10px] bg-foreground p-1 shadow-sm",
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
	className,
	...props
}: TabsPrimitive.TabsTriggerProps) {
	return (
		<TabsPrimitive.Trigger
			className={cn(
				"flex grow items-center justify-center rounded-md border border-transparent bg-transparent px-4 py-2 text-text",
				"transition-[background-color] duration-100 ease-in-out hover:cursor-pointer",
				"data-[state=active]:bg-primary-background data-[state=active]:text-primary",
				className,
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
