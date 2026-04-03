"use client";

import * as React from "react";
import * as AvatarPrimitive from "@radix-ui/react-avatar";
import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

type AvatarProps = React.ComponentProps<typeof AvatarPrimitive.Root> & {
	shape?: "circle" | "square-small" | "square-medium";
	size: number;
};

const avatarStyles = cva(
	cn(
		"inline-flex items-center justify-center align-middle",
		"h-full w-full overflow-hidden bg-surface-overlay select-none",
	),
	{
		variants: {
			shape: {
				circle: "rounded-full",
				"square-small": "rounded-md",
				"square-medium": "rounded-[10px]",
			},
		},
		defaultVariants: {
			shape: "circle",
		},
	},
);

export function Avatar({ size, shape, className, ...props }: AvatarProps) {
	return (
		<AvatarPrimitive.Root
			style={{ width: size, height: size }}
			className={avatarStyles({ shape, className })}
			{...props}
		/>
	);
}

Avatar.displayName = AvatarPrimitive.Root.displayName;

export function AvatarImage({
	className,
	...props
}: React.ComponentProps<typeof AvatarPrimitive.Image>) {
	return (
		<AvatarPrimitive.Image
			className={cn("h-full w-full object-cover", className)}
			{...props}
		/>
	);
}

AvatarImage.displayName = AvatarPrimitive.Image.displayName;

export function AvatarFallback({
	className,
	...props
}: React.ComponentProps<typeof AvatarPrimitive.Fallback>) {
	return (
		<AvatarPrimitive.Fallback
			className={cn(
				"flex h-full w-full text-center justify-center items-center",
				className,
			)}
			{...props}
		/>
	);
}

AvatarFallback.displayName = AvatarPrimitive.Fallback.displayName;
