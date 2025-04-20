import React from "react";
import { Button } from "@/components/atoms/buttons/button";

export default function LoginPage() {
	return (
		<div>
			<h1>Login</h1>
			<form>
				<div>
					<label htmlFor="username">Username</label>
					<input type="text" id="username" name="username" required />
				</div>
				<div>
					<label htmlFor="password">Password</label>
					<input
						type="password"
						id="password"
						name="password"
						required
					/>
				</div>
				<Button type="submit" level="primary">
					Login
				</Button>
			</form>
		</div>
	);
}
