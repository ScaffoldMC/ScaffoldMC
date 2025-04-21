import React from "react";
import { Button } from "@/components/atoms/buttons/button";
import styles from "./styles.module.css";
import { Input } from "@/components/atoms/input/input";
import { Label } from "@/components/atoms/label/label";

export default function LoginPage() {
	return (
		<div className={styles.root}>
			<div className={styles.login}>
				<h2>Sign In</h2>
				<form className={styles.form}>
					<div className={styles.field}>
						<Label htmlFor="username">Username</Label>
						<Input
							type="text"
							id="username"
							name="username"
							required
						/>
					</div>
					<div className={styles.field}>
						<Label htmlFor="password">Password</Label>
						<Input
							type="password"
							id="password"
							name="password"
							required
						/>
					</div>
					<Button type="submit" level="primary">
						Sign In
					</Button>
				</form>
			</div>
		</div>
	);
}
