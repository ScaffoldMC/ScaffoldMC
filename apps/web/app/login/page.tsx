import React from "react";
import { Button } from "@/components/atoms/buttons/button";
import styles from "./styles.module.css";
import { Input } from "@/components/atoms/input/input";
import { Label } from "@/components/atoms/label/label";

export default function LoginPage() {
	return (
		<div className={styles.root}>
			<div className={styles.loginContainer}>
				<h2>Login</h2>
				<form>
					<div>
						<Label htmlFor="username">Username</Label>
						<Input
							type="text"
							id="username"
							name="username"
							required
						/>
					</div>
					<div>
						<Label htmlFor="password">Password</Label>
						<Input
							type="password"
							id="password"
							name="password"
							required
						/>
					</div>
					<Button
						type="submit"
						level="primary"
						style={{ width: "100%" }}
					>
						Login
					</Button>
				</form>
			</div>
		</div>
	);
}
