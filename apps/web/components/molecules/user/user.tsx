import { Avatar, AvatarFallback } from "@/components/atoms/avatar/avatar";
import styles from "./user.module.css";

export function User() {
	// TODO: Get user data from server's API

	return (
		<div className={styles.root}>
			<Avatar size={40} className={styles.avatar}>
				<AvatarFallback>JD</AvatarFallback>
			</Avatar>
			<div className={styles.details}>
				<b>John Doe</b>
				<p className={styles.email}>john.doe@example.com</p>
			</div>
		</div>
	);
}
