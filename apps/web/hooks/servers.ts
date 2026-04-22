import api from "@/lib/axios";
import { CreateServerRequest } from "@/lib/servertypes";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { PartialServerConfig } from "../../backend/bindings/PartialServerConfig";

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
	const queryClient = useQueryClient();

	const server = useQuery({
		queryKey: ["server", serverId],
		queryFn: () => api.get(`/servers/${serverId}`).then((res) => res.data),
		refetchInterval: 5000,
		retry: false,
	});

	const isRunning = server.data?.state === "Running";
	const isStarting = server.data?.state === "Starting";

	const sendCommand = async (command: string) => {
		await api.post(`/servers/${serverId}/console`, { command });
	};

	const startServer = async () => {
		await api.post(`/servers/${serverId}/start`);
		await queryClient.invalidateQueries({ queryKey: ["server", serverId] });
		await queryClient.invalidateQueries({ queryKey: ["servers"] });
	};

	const stopServer = async () => {
		await api.post(`/servers/${serverId}/stop`);
		await queryClient.invalidateQueries({ queryKey: ["server", serverId] });
		await queryClient.invalidateQueries({ queryKey: ["servers"] });
	};

	const mutateConfig = useMutation({
		mutationFn: async (config: PartialServerConfig) =>
			await api.patch(`/servers/${serverId}/config`, config),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ["server", serverId] });
			queryClient.invalidateQueries({ queryKey: ["servers"] });
		},
	}).mutateAsync;

	const deleteServer = async () => {
		await api.delete(`/servers/${serverId}`);
		await queryClient.invalidateQueries({ queryKey: ["server", serverId] });
		await queryClient.invalidateQueries({ queryKey: ["servers"] });
	};

	return {
		server,
		isStarting,
		isRunning,
		sendCommand,
		startServer,
		stopServer,
		deleteServer,
		mutateConfig,
	};
}
