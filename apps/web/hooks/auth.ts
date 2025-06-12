"use client";

import api from "@/lib/axios";
import { LoginRequest, UserResponse } from "@/lib/servertypes";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export function useAuth() {
	const queryClient = useQueryClient();
	const router = useRouter();

	const user = useQuery({
		queryKey: ["me"],
		queryFn: () => api.get("/auth/me").then((res) => res.data),
		retry: false,
	});

	const loginMutation = useMutation({
		mutationFn: async (credentials: LoginRequest) =>
			await api.post("/auth/login", credentials),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ["me"] });
			router.push("/dashboard");
		},
	});

	const logoutMutation = useMutation({
		mutationFn: async () => {
			await api.post("/auth/logout");
			queryClient.invalidateQueries({ queryKey: ["me"] });
			router.push("/login");
		},
	});

	const authenticated = Boolean(user.data) && !user.isError;

	return {
		authenticated,
		login: loginMutation.mutateAsync,
		logout: logoutMutation.mutateAsync,
	};
}

export function useUser() {
	const user = useQuery<UserResponse>({
		queryKey: ["me"],
		queryFn: () => api.get("/auth/me").then((res) => res.data),
		retry: false,
	});

	const router = useRouter();

	useEffect(() => {
		if (user.isError) {
			router.push("/login");
		}
	}, [user.isError, router]);

	return {
		loading: user.isLoading,
		user: user.data,
	};
}
