"use client";

import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Button } from "@/components/atoms/Button/Button";
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
		<div className="flex w-100 flex-col items-center justify-center gap-6 rounded-lg border border-border-static bg-surface-raised p-12 text-text-primary">
			<h3 className="text-xl">Sign in to continue</h3>
			<form
				className="flex w-full flex-col items-center justify-center gap-6 [&_button]:w-full [&_input]:w-full"
				onSubmit={handleSubmit}
			>
				<div className="flex w-full flex-col items-start justify-start gap-1">
					<Label htmlFor="username">Username</Label>
					<TextInput
						type="text"
						id="username"
						name="username"
						invalid={isError}
						required
					/>
				</div>
				<div className="flex w-full flex-col items-start justify-start gap-1">
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
