import { cn } from "@/lib/util";

export function Dropdown({
	children,
	className,
	...props
}: React.SelectHTMLAttributes<HTMLSelectElement>) {
	return (
		<select
			className={cn(
				"rounded-lg border border-border bg-foreground p-1.5 text-sm text-text",
				"transition-[background-color,border-color] duration-100 ease-in-out",
				"hover:border-border-hover",
				className,
			)}
			{...props}
		>
			{children}
		</select>
	);
}
