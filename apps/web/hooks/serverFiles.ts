import api from "@/lib/axios";
import { FSEntry } from "@/lib/servertypes";
import { useQuery, useQueryClient } from "@tanstack/react-query";

function normalizePath(path: string) {
	return path.replace(/^\/+/, "");
}

function getServerFilesPath(serverId: string, path: string) {
	const normalizedPath = normalizePath(path);
	return normalizedPath
		? `/servers/${serverId}/files/${normalizedPath}`
		: `/servers/${serverId}/files`;
}

export function useServerFilesystem(serverId: string) {
	const queryClient = useQueryClient();
	const queryKey = ["server-files", serverId] as const;

	const invalidateFilesystem = () =>
		queryClient.invalidateQueries({ queryKey });

	const fetchEntry = async <T>(
		path: string,
		params?: Record<string, string>,
	) => {
		const { data } = await api.get<T>(getServerFilesPath(serverId, path), {
			params,
		});
		return data;
	};

	const mutateFilesystem = async (request: () => Promise<void>) => {
		await request();
		await invalidateFilesystem();
	};

	const filesystem = useQuery({
		queryKey: [...queryKey, "dir", "/"],
		queryFn: (): Promise<FSEntry[]> =>
			fetchEntry<FSEntry[]>("", { content: "1" }),
		enabled: Boolean(serverId),
		retry: false,
	});

	const getMetadata = (path: string) => fetchEntry<FSEntry>(path);
	const listDirectory = (path: string) =>
		fetchEntry<FSEntry[]>(path, { content: "1" });
	const readFile = async (path: string) => {
		const { data } = await api.get<string>(
			getServerFilesPath(serverId, path),
			{
				params: { content: "1" },
				responseType: "text",
			},
		);
		return data;
	};

	const createFile = async (path: string) =>
		mutateFilesystem(async () => {
			await api.post(getServerFilesPath(serverId, path), null, {
				params: { type: "file" },
			});
		});

	const createDirectory = async (path: string) =>
		mutateFilesystem(async () => {
			await api.post(getServerFilesPath(serverId, path), null, {
				params: { type: "directory" },
			});
		});

	const renameEntry = async ({ path, to }: { path: string; to: string }) =>
		mutateFilesystem(async () => {
			await api.put(getServerFilesPath(serverId, path), undefined, {
				params: {
					operation: "rename",
					to,
				},
			});
		});

	const relocateEntry = async ({ path, to }: { path: string; to: string }) =>
		mutateFilesystem(async () => {
			await api.put(getServerFilesPath(serverId, path), undefined, {
				params: {
					operation: "move",
					to,
				},
			});
		});

	const writeFile = async ({
		path,
		content,
	}: {
		path: string;
		content: string | Blob | ArrayBuffer;
	}) =>
		mutateFilesystem(async () => {
			await api.put(getServerFilesPath(serverId, path), content, {
				params: {
					operation: "write",
				},
			});
		});

	const deleteEntry = async (path: string) =>
		mutateFilesystem(async () => {
			await api.delete(getServerFilesPath(serverId, path));
		});

	return {
		filesystem,
		invalidateFilesystem,
		getMetadata,
		listDirectory,
		readFile,
		createFile,
		createDirectory,
		writeFile,
		renameEntry,
		relocateEntry,
		deleteEntry,
	};
}
