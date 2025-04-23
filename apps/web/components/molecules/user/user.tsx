import { Avatar, AvatarFallback } from "@/components/atoms/avatar/avatar";
import styles from "./user.module.css";
import { LogOut } from "lucide-react";
import { Button } from "@/components/atoms/buttons/button";

export function User() {
	// TODO: Get user data from server's API

	return (
		<div className={styles.root}>
			<Avatar size={40}>
				<AvatarFallback>JD</AvatarFallback>
			</Avatar>
			<div className={styles.details}>
				<b>John Doe</b>
			</div>
			<Button size="icon" level="secondary">
				<LogOut size={16} />
			</Button>
		</div>
	);
}
