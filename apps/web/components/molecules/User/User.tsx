"use client";

import { Avatar, AvatarFallback } from "@/components/atoms/Avatar/Avatar";
import { LogOut } from "lucide-react";
import { Button } from "@/components/atoms/Button/Button";
import { useAuth, useLogout } from "@/hooks/auth";
import { makeInitials } from "@/lib/util";

// TODO: Abstract out business logic

export function User() {
	const logout = useLogout();
	const { user, authenticated, isLoading } = useAuth();

	// TODO: Display user profile picture

	if (!authenticated && !isLoading) {
		return null;
	}

	// TODO: Create loading skeleton
	if (isLoading) {
		return null;
	}

	return (
		<div className="flex w-full flex-row items-center gap-2.5 [&>*:last-child]:ml-auto">
			<Avatar size={40}>
				<AvatarFallback>{makeInitials(user.fullname)}</AvatarFallback>
			</Avatar>
			<div className="flex flex-col items-start justify-start">
				<b>{user.fullname}</b>
			</div>
			<Button level="secondary" onClick={() => logout()}>
				<LogOut size={18} />
			</Button>
		</div>
	);
}
