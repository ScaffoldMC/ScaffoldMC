"use client";

import {
	Avatar,
	AvatarFallback,
	AvatarImage,
} from "@/components/atoms/Avatar/Avatar";

import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from "@/components/organisms/Tabs/Tabs";

import { useQuery } from "@tanstack/react-query";
import api from "@/lib/axios";
import { useParams, useRouter } from "next/navigation";
import { ServerInfo } from "@/lib/servertypes";

export default function Page() {
	const { slug } = useParams();

	const server = useQuery({
		queryKey: ["server", slug],
		queryFn: (): Promise<ServerInfo> =>
			api.get(`/servers/${slug}`).then((res) => res.data),
		retry: false,
	});

	const router = useRouter();

	if (!server.data && !server.isLoading) {
		router.push("/404");
		return null;
	}

	if (!server.data) {
		return <div>Loading...</div>;
	}

	return (
		<div className="flex flex-col gap-4">
			<div className="flex items-center gap-4">
				<Avatar size={64} shape="square-medium">
					<AvatarFallback>?</AvatarFallback>
					<AvatarImage src="/images/server-default.png" />
				</Avatar>
				<div className="flex flex-col">
					<h1>{server.data.name}</h1>
					<p>{server.data.state}</p>
				</div>
			</div>

			<Tabs defaultValue="console" className="gap-2">
				<TabsList>
					<TabsTrigger value="console">Console</TabsTrigger>
					<TabsTrigger value="settings">Settings</TabsTrigger>
					<TabsTrigger value="backups">Backups</TabsTrigger>
				</TabsList>
				<TabsContent value="console">
					<div className="bg-surface p-2 min-h-100 w-full rounded-md border border-border-static">
						<p>hello</p>
					</div>
				</TabsContent>
				<TabsContent value="settings">
					<b>Settings</b>
				</TabsContent>
				<TabsContent value="backups">
					<b>Backups</b>
				</TabsContent>
			</Tabs>
		</div>
	);
}
