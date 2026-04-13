"use client";

import Link from "next/link";
import { List, ListItem } from "@/components/organisms/List/List";
import { useQuery } from "@tanstack/react-query";
import api from "@/lib/axios";
import Image from "next/image";
import { cn } from "@/lib/util";
import { ArrowRight } from "lucide-react";

export function ServerListPreview() {
	const serverIds = useQuery({
		queryKey: ["servers"],
		queryFn: () => api.get("/servers").then((res) => res.data),
		retry: false,
	});

	return (
		<List hideHeader={true}>
			{serverIds.data?.map((uuid: string) => (
				<ServerListPreviewItem key={uuid} uuid={uuid} />
			))}
			<Link
				href="/servers"
				className="flex items-center gap-2 p-2 px-3 text-md text-primary rounded hover:bg-surface-raised transition-colors"
			>
				Go to servers page <ArrowRight size={18} />
			</Link>
		</List>
	);
}

function ServerListPreviewItem({ uuid }: { uuid: string }) {
	const serverInfo = useQuery({
		queryKey: ["server", uuid],
		queryFn: () => api.get(`/servers/${uuid}`).then((res) => res.data),
		retry: false,
	});

	return (
		<ListItem>
			<Link
				href={`/servers/${uuid}`}
				className="font-medium text-text-primary no-underline"
			>
				<div
					className={cn(
						"flex flex-row items-center justify-between p-2",
						"transition-[background-color] duration-100 ease-in-out hover:bg-surface-raised",
					)}
				>
					<div className="flex w-fit flex-row items-center gap-2">
						<Image
							src="/images/server-default.png"
							alt="Server image"
							width={36}
							height={36}
							className="rounded-md"
						/>

						<p>{serverInfo.data?.config.name || "Server name"}</p>
					</div>
				</div>
			</Link>
		</ListItem>
	);
}
