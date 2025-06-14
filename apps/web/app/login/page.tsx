"use client";

import React from "react";
import styles from "./styles.module.css";
import { Login } from "@/components/organisms/login/login";
import { useLogin } from "@/hooks/auth";
import { useRouter } from "next/navigation";

export default function LoginPage() {
	const login = useLogin();
	const router = useRouter();

	const handleLogin = async (username: string, password: string) => {
		await login({ username, password });
		router.push("/home");
	};

	return (
		<div className={styles.root}>
			<Login onLogin={handleLogin} />
		</div>
	);
}
