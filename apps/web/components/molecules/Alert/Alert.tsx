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
				info: "border-accent-base bg-accent-lightest text-accent-darkest",
				warning: "border-brand bg-brand/25 text-text-primary",
				error: "border-red-500 bg-red-100 text-red-900",
			},
		},
	})({ type });

	return <div className={className}>{children}</div>;
}
