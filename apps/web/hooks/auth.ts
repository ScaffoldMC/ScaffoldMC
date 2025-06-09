import api from "@/lib/axios";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { setAccessToken } from "@/lib/accesstoken";

export const useAuth = () => {
	const queryClient = useQueryClient();
	const router = useRouter();

	const user = useQuery({
		queryKey: ["me"],
		queryFn: () => api.get("/auth/me").then((res) => res.data),
		retry: false,
	});

	const loginMutation = useMutation({
		mutationFn: async (credentials: {
			email: string;
			password: string;
		}) => {
			const res = await api.post("/auth/login", credentials);
			setAccessToken(res.data.accessToken);
			return res.data;
		},
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ["me"] });
			router.push("/dashboard");
		},
	});

	const logoutMutation = useMutation({
		mutationFn: async () => {
			await api.post("/auth/logout");
			setAccessToken("");
			queryClient.invalidateQueries({ queryKey: ["me"] });
			router.push("/login");
		},
	});

	const authenticated = Boolean(user.data) && !user.isError;

	return {
		authenticated,
		isLoading: user.isLoading,
		user,
		login: loginMutation.mutateAsync,
		logout: logoutMutation.mutateAsync,
	};
};
