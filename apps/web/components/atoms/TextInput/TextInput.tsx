import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

const inputVariants = cva(
	cn(
		"h-9 w-fit rounded-md border border-border bg-transparent px-2",
		"text-sm text-text placeholder:text-neutral-400",
		"transition-[border-color] duration-100 ease-in-out",
		"hover:border-border-hover focus:border-text focus:outline-none",
		"disabled:hover:cursor-not-allowed disabled:hover:border-border",
	),
	{
		variants: {
			invalid: {
				true: "outline-1 outline-error",
				false: "",
			},
		},
		defaultVariants: {
			invalid: false,
		},
	},
);

export type TextInputProps = Omit<
	React.InputHTMLAttributes<HTMLInputElement>,
	"type"
> & {
	type?: "text" | "email" | "password" | "search" | "tel" | "url";
	invalid?: boolean;
};

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
