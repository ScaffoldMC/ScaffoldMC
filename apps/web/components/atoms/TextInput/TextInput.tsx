import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

const inputVariants = cva(
	cn(
		"w-fit focus:outline-none",
		"text-sm text-text-primary placeholder:text-text-tertiary",
		"disabled:hover:cursor-not-allowed",
	),
	{
		variants: {
			variant: {
				normal: cn(
					"h-9 rounded-md border border-border-static bg-surface-raised px-2",
					"hover:border-border-hover focus:border-border-active",
					"transition-[border-color] duration-100 ease-in-out",
					"disabled:hover:border-border-static",
				),
				ghost: "",
			},
			invalid: {
				true: "border-red-500 focus:border-red-600",
				false: "",
			},
		},
		defaultVariants: {
			invalid: false,
			variant: "normal",
		},
	},
);

export interface TextInputProps
	extends Omit<React.ComponentPropsWithRef<"input">, "type"> {
	type?: "text" | "email" | "password" | "search" | "tel" | "url";
	invalid?: boolean;
	variant?: "normal" | "ghost";
}

export function TextInput({
	type,
	invalid,
	className,
	variant,
	...props
}: TextInputProps) {
	return (
		<input
			type={type}
			className={inputVariants({ invalid, className, variant })}
			{...props}
		/>
	);
}
