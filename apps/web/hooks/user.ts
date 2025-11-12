"use client";

import api from "@/lib/axios";
import { UserPatchRequest } from "@/lib/servertypes";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export function useCurrentUser() {
	const queryClient = useQueryClient();
	const user = useQuery({
		queryKey: ["me"],
		queryFn: () => api.get("/me").then((res) => res.data),
		retry: false,
	});

	const mutationHook = useMutation({
		mutationFn: async (updates: Partial<UserPatchRequest>) => {
			const { data } = await api.patch("/me", updates);
			return data;
		},
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ["me"] });
		},
	}).mutateAsync;

	return { user, mutationHook };
}
