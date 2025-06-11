"use client";

import { Label } from "@/components/atoms/label/label";
import { TextInput } from "@/components/atoms/input/textinput";
import { Button } from "@/components/atoms/buttons/button";
import { Checkbox } from "@/components/atoms/checkbox/checkbox";
import styles from "./login.module.css";
import { useAuth } from "@/hooks/auth";
import { useRouter } from "next/navigation";

export function Login() {
	const { login } = useAuth();
	const router = useRouter();

	const handleLogin = (event: React.FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		const formData = new FormData(event.currentTarget);
		const username = formData.get("username") as string;
		const password = formData.get("password") as string;

		login({ username, password })
			.then(() => {
				// TODO: Display success message
				router.push("/home");
			})
			.catch((error) => {
				// TODO: Make field items red to indicate error
			});
	};

	return (
		<div className={styles.login}>
			<h2>Sign In</h2>
			<form className={styles.form} onSubmit={handleLogin}>
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
				<div className={styles.line}>
					<Checkbox id="remember" />
					<Label htmlFor="remember">Remember Me</Label>
				</div>
				<Button type="submit" level="primary">
					Sign In
				</Button>
			</form>
		</div>
	);
}
