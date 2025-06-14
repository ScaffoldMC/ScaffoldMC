"use client";

import { Label } from "@/components/atoms/label/label";
import { TextInput } from "@/components/atoms/input/textinput";
import { Button } from "@/components/atoms/buttons/button";
import styles from "./login.module.css";

interface LoginProps {
	onLogin?: (username: string, password: string) => Promise<void>;
}

export function Login({ onLogin }: LoginProps) {
	const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		const formData = new FormData(event.currentTarget);
		const username = formData.get("username") as string;
		const password = formData.get("password") as string;

		onLogin(username, password).catch(() => {
			// TODO: Make field items red to indicate error
		});
	};

	return (
		<div className={styles.login}>
			<h3>Sign in to continue</h3>
			<form className={styles.form} onSubmit={handleSubmit}>
				<div className={styles.field}>
					<Label htmlFor="username">Username</Label>
					<TextInput
						type="text"
						id="username"
						name="username"
						required
					/>
				</div>
				<div className={styles.field}>
					<Label htmlFor="password">Password</Label>
					<TextInput
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
	);
}
