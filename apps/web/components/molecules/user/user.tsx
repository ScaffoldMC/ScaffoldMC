"use client";

import { Avatar, AvatarFallback } from "@/components/atoms/avatar/avatar";
import styles from "./user.module.css";
import { LogOut } from "lucide-react";
import { Button } from "@/components/atoms/buttons/button";
import { useAuth, useLogout } from "@/hooks/auth";

export function User() {
	const logout = useLogout();
	const { user, authenticated, isLoading } = useAuth();

	// TODO: Display user profile picture

	if (!authenticated && !isLoading) {
		return null;
	}

	// TODO: Create loading skeleton

	return (
		<div className={styles.root}>
			<Avatar size={40}>
				<AvatarFallback>JD</AvatarFallback>
			</Avatar>
			<div className={styles.details}>
				<b>{user?.fullname || ""}</b>
			</div>
			<Button size="icon" level="secondary" onClick={() => logout()}>
				<LogOut size={16} />
			</Button>
		</div>
	);
}
