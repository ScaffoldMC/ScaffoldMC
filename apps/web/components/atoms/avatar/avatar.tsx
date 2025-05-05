"use client";

import * as React from "react";
import * as AvatarPrimitive from "@radix-ui/react-avatar";
import styles from "./avatar.module.css";
import { cva } from "class-variance-authority";

type AvatarProps = React.ComponentProps<typeof AvatarPrimitive.Root> & {
	shape?: "circle" | "square-small" | "square-medium";
	size: number;
};

const avatarStyles = cva(styles.base, {
	variants: {
		shape: {
			circle: styles.circle,
			"square-small": styles.squareSmall,
			"square-medium": styles.squareMedium,
		},
	},
	defaultVariants: {
		shape: "circle",
	},
});

export function Avatar({ size, shape, ...props }: AvatarProps) {
	return (
		<AvatarPrimitive.Root
			style={{ width: size, height: size }}
			className={avatarStyles({ shape })}
			{...props}
		/>
	);
}

Avatar.displayName = AvatarPrimitive.Root.displayName;

export function AvatarImage({
	...props
}: React.ComponentProps<typeof AvatarPrimitive.Image>) {
	return <AvatarPrimitive.Image {...props} />;
}

AvatarImage.displayName = AvatarPrimitive.Image.displayName;

export function AvatarFallback({
	...props
}: React.ComponentProps<typeof AvatarPrimitive.Fallback>) {
	return <AvatarPrimitive.Fallback {...props} />;
}

AvatarFallback.displayName = AvatarPrimitive.Fallback.displayName;
