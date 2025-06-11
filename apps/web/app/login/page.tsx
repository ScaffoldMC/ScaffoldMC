import React from "react";
import styles from "./styles.module.css";
import { Login } from "@/components/organisms/login/login";

export default function LoginPage() {
	return (
		<div className={styles.root}>
			<Login />
		</div>
	);
}
