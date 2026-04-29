import * as TabsPrimitive from "@radix-ui/react-tabs";
import { cn } from "@/lib/util";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";

export function HashTabs({
	children,
	className,
	defaultValue,
	...props
}: TabsPrimitive.TabsProps) {
	const router = useRouter();
	const [activeTab, setActiveTab] = useState(defaultValue ?? "");

	useEffect(() => {
		const hash = window.location.hash.replace("#", "");
		if (hash) setActiveTab(hash);

		const onHashChange = () => {
			setActiveTab(hash || defaultValue || "");
		};

		window.addEventListener("hashchange", onHashChange);
		return () => window.removeEventListener("hashchange", onHashChange);
	}, [defaultValue]);

	return (
		<TabsPrimitive.Root
			className={cn("flex w-full flex-col items-center", className)}
			value={activeTab}
			onValueChange={(value) => {
				setActiveTab(value);
				router.push(`#${value}`, { scroll: false });
			}}
			{...props}
		>
			{children}
		</TabsPrimitive.Root>
	);
}

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
				"h-8 flex-1 items-center justify-center rounded-md bg-transparent px-4 py-2",
				"flex flex-row gap-2 items-center justify-center",
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
	className,
	...props
}: TabsPrimitive.TabsContentProps) {
	return (
		<TabsPrimitive.Content {...props} className={cn("w-full", className)}>
			{children}
		</TabsPrimitive.Content>
	);
}
