import { cn } from "@/lib/util";
import { Ref, SelectHTMLAttributes } from "react";

export function Dropdown({
	children,
	className,
	ref,
	...props
}: {
	children: React.ReactNode;
	className: string;
	ref: Ref<HTMLSelectElement>;
} & SelectHTMLAttributes<HTMLSelectElement>) {
	return (
		<select
			ref={ref}
			className={cn(
				"rounded-lg border border-border-static bg-surface-raised p-1.5 text-sm text-text-primary",
				"transition-[background-color,border-color] duration-100 ease-in-out",
				"hover:border-border-hover focus:border-border-active focus:outline-none",
				className,
			)}
			{...props}
		>
			{children}
		</select>
	);
}
