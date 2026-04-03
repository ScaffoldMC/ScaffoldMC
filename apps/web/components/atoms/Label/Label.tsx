import { cn } from "@/lib/util";

export function Label({
	className,
	...props
}: React.LabelHTMLAttributes<HTMLLabelElement>) {
	return (
		<label
			className={cn("text-md text-text-primary", className)}
			{...props}
		/>
	);
}
