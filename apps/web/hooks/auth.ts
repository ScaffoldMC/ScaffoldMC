"use client";

import api from "@/lib/axios";
import { LoginRequest, UserResponse } from "@/lib/servertypes";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export function useLogin() {
	const queryClient = useQueryClient();
	const router = useRouter();

	return useMutation({
		mutationFn: async (credentials: LoginRequest) =>
			await api.post("/auth/login", credentials),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ["me"] });
			router.push("/home");
		},
	}).mutateAsync;
}

export function useLogout() {
	const queryClient = useQueryClient();
	const router = useRouter();

	return useMutation({
		mutationFn: async () => {
			await api.post("/auth/logout");
			queryClient.invalidateQueries({ queryKey: ["me"] });
			router.push("/login");
		},
	}).mutateAsync;
}

export function useAuth() {
	const router = useRouter();
	const queryClient = useQueryClient();
	const user = useQuery({
		queryKey: ["me"],
		queryFn: () => api.get("/me").then((res) => res.data),
		retry: false,
	});

	const authenticated = Boolean(user.data) && !user.isError;

	useEffect(() => {
		if (user.isError && !user.isLoading) {
			queryClient.removeQueries({ queryKey: ["me"] });
			router.replace("/login");
		}
	}, [user.isError, user.isLoading]);

	return {
		user: user.data as UserResponse | null,
		authenticated,
		isLoading: user.isLoading,
	};
}
