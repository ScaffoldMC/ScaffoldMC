import api from "@/lib/axios";
import { CreateServerRequest } from "@/lib/servertypes";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export function useServers() {
	const queryClient = useQueryClient();

	const servers = useQuery({
		queryKey: ["servers"],
		queryFn: () => api.get("/servers").then((res) => res.data),
		retry: false,
	});

	const mutateServers = useMutation({
		mutationFn: async (createRequest: CreateServerRequest) =>
			await api.post("/servers", createRequest),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ["servers"] });
		},
	}).mutateAsync;

	return { servers, mutateServers };
}

export function useServer(serverId: string) {
	const server = useQuery({
		queryKey: ["server", serverId],
		queryFn: () => api.get(`/servers/${serverId}`).then((res) => res.data),
		retry: false,
	});

	const isRunning = server.data?.state === "Running";

	// TODO: Mutate server

	return { server, isRunning };
}
