import React from "react";
import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

interface AlertProps extends React.HTMLAttributes<HTMLDivElement> {
	type: "info" | "warning" | "error";
}

export function Alert({ children, type }: AlertProps) {
	const className = cva(
		cn("flex flex-row gap-2 items-center rounded-lg border p-2"),
		{
			variants: {
				type: {
					info: "border-accent-base bg-accent-lightest text-accent-darkest",
					warning: cn(
						"border-yellow-500 bg-yellow-100 text-yellow-900",
						"dark:border-yellow-500 dark:bg-yellow-900/10 dark:text-yellow-300",
					),
					error: "border-red-500 bg-red-100 text-red-900",
				},
			},
		},
	)({ type });

	return <div className={className}>{children}</div>;
}
