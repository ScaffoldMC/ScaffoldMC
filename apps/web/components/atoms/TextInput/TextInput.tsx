import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

const inputVariants = cva(
	cn(
		"h-9 w-fit rounded-md border border-border-static bg-surface-raised px-2",
		"text-sm text-text-primary placeholder:text-text-tertiary",
		"transition-[border-color] duration-100 ease-in-out",
		"hover:border-border-hover focus:border-border-active focus:outline-none",
		"disabled:hover:cursor-not-allowed disabled:hover:border-border-static",
	),
	{
		variants: {
			invalid: {
				true: "border-red-500 focus:border-red-600",
				false: "",
			},
		},
		defaultVariants: {
			invalid: false,
		},
	},
);

export interface TextInputProps
	extends Omit<React.ComponentPropsWithRef<"input">, "type"> {
	type?: "text" | "email" | "password" | "search" | "tel" | "url";
	invalid?: boolean;
}

export function TextInput({
	type,
	invalid,
	className,
	...props
}: TextInputProps) {
	return (
		<input
			type={type}
			className={inputVariants({ invalid, className })}
			{...props}
		/>
	);
}
