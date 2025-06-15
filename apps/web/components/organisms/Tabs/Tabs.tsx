import * as TabsPrimitive from "@radix-ui/react-tabs";
import styles from "./Tabs.module.css";

export function Tabs({ children, ...props }: TabsPrimitive.TabsProps) {
	return (
		<TabsPrimitive.Root className={styles.root} {...props}>
			{children}
		</TabsPrimitive.Root>
	);
}

export function TabsList({ children, ...props }: TabsPrimitive.TabsListProps) {
	return (
		<TabsPrimitive.List className={styles.list} {...props}>
			{children}
		</TabsPrimitive.List>
	);
}

export function TabsTrigger({
	children,
	...props
}: TabsPrimitive.TabsTriggerProps) {
	return (
		<TabsPrimitive.Trigger className={styles.trigger} {...props}>
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
