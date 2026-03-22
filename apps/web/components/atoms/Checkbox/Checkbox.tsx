import {
	Root,
	CheckboxIndicator,
	CheckboxProps,
} from "@radix-ui/react-checkbox";

import { Check } from "lucide-react";
import { cn } from "@/lib/util";

export function Checkbox({ className, ...props }: CheckboxProps) {
	return (
		<Root
			className={cn(
				"inline-flex size-6 items-center justify-center rounded border border-border-static",
				"bg-transparent transition-[background-color,border-color] duration-100 ease-in-out",
				"hover:border-border-hover data-[state=checked]:border-accent-base data-[state=checked]:bg-accent-base",
				"disabled:cursor-not-allowed",
				className,
			)}
			{...props}
		>
			<CheckboxIndicator className="flex h-full w-full items-center justify-center text-surface-raised">
				<Check />
			</CheckboxIndicator>
		</Root>
	);
}
