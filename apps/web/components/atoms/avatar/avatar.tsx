"use client";

import * as React from "react";
import * as AvatarPrimitive from "@radix-ui/react-avatar";
import styles from "./avatar.module.css";

export function Avatar({
	...props
}: React.ComponentProps<typeof AvatarPrimitive.Root>) {
	return <AvatarPrimitive.Root className={styles.base} {...props} />;
}

Avatar.displayName = AvatarPrimitive.Root.displayName;

export function AvatarImage({
	...props
}: React.ComponentProps<typeof AvatarPrimitive.Image>) {
	return <AvatarPrimitive.Image className={styles.base} {...props} />;
}

AvatarImage.displayName = AvatarPrimitive.Image.displayName;

export function AvatarFallback({
	...props
}: React.ComponentProps<typeof AvatarPrimitive.Fallback>) {
	return (
		<AvatarPrimitive.Fallback
			className={(styles.base, styles.fallback)}
			{...props}
		/>
	);
}

AvatarFallback.displayName = AvatarPrimitive.Fallback.displayName;
