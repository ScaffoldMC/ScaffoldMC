import { Button } from "@/components/atoms/Button/Button";
import { cn } from "@/lib/util";
import { cva } from "class-variance-authority";

const buttonStyles = cva(
	cn(
		"w-full text-sm text-text-secondary font-light",
		"inline-flex items-center justify-start cursor-pointer",
		"h-9 gap-2 rounded-md border px-2",
		"transition-[background-color,border-color,color] duration-100 ease-in-out",
		"[&_svg]:inline-block disabled:cursor-not-allowed disabled:opacity-50",
	),
	{
		variants: {
			selected: {
				true: cn(
					"text-text-primary bg-accent-lightest dark:bg-accent-darkest",
				),
				false: "hover:bg-accent-darkest hover:text-text-primary",
			},
		},
		defaultVariants: {
			selected: false,
		},
	},
);

export interface FilesListButtonProps {
	selected: boolean;
	onClick: () => void;
	children: React.ReactNode;
}

export function FilesListButton({
	selected,
	onClick,
	children,
}: FilesListButtonProps) {
	const className = buttonStyles({ selected });

	return (
		<Button level="ghost" className={className} onClick={onClick}>
			{children}
		</Button>
	);
}
