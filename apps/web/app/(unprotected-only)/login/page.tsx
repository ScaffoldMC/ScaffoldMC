"use client";

import { Login } from "@/components/organisms/Login/Login";
import { useLogin } from "@/hooks/auth";
import { useRouter } from "next/navigation";

export default function LoginPage() {
	const login = useLogin();
	const router = useRouter();

	const handleLogin = async (username: string, password: string) => {
		await login({ username, password });
		router.push("/servers");
	};

	return (
		<div className="flex flex-col gap-2 h-lvh w-full items-center justify-center">
			<Login onLogin={handleLogin} />
		</div>
	);
}
