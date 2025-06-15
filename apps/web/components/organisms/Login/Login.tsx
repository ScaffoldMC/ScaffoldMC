"use client";

import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Button } from "@/components/atoms/Button/Button";
import styles from "./Login.module.css";
import { useState } from "react";

interface LoginProps {
	onLogin?: (username: string, password: string) => Promise<void>;
}

export function Login({ onLogin }: LoginProps) {
	const [isError, setIsError] = useState(false);

	const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		const formData = new FormData(event.currentTarget);
		const username = formData.get("username") as string;
		const password = formData.get("password") as string;

		onLogin(username, password).catch(() => {
			setIsError(true);
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
						invalid={isError}
						required
					/>
				</div>
				<div className={styles.field}>
					<Label htmlFor="password">Password</Label>
					<TextInput
						type="password"
						id="password"
						name="password"
						invalid={isError}
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
