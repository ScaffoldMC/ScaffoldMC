import React from "react";
import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

interface AlertProps extends React.HTMLAttributes<HTMLDivElement> {
	type: "info" | "warning" | "error";
}

export function Alert({ children, type }: AlertProps) {
	const className = cva(cn("rounded-lg border p-2"), {
		variants: {
			type: {
				info: "border-sky-500 bg-sky-100 text-sky-900 dark:border-sky-700 dark:bg-sky-900 dark:text-sky-200",
				warning:
					"border-amber-400 bg-amber-50 text-amber-800 dark:border-amber-600 dark:bg-amber-800 dark:text-amber-100",
				error: "border-rose-400 bg-rose-100 text-rose-900 dark:border-rose-300 dark:bg-rose-950 dark:text-rose-200",
			},
		},
	})({ type });

	return <div className={className}>{children}</div>;
}
