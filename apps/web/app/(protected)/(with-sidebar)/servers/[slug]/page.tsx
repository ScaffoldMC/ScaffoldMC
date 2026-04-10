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
import { ServerConsole } from "@/components/organisms/ServerConsole/ServerConsole";
import { ServerStartStopButton } from "@/components/organisms/ServerStartStopButton/ServerStartStopButton";
import PageLayout from "@/components/atoms/PageLayout/PageLayout";

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
		<PageLayout>
			<div className="relative flex items-center gap-4">
				<Avatar size={64} shape="square-medium">
					<AvatarFallback>?</AvatarFallback>
					<AvatarImage src="/images/server-default.png" />
				</Avatar>
				<div className="flex flex-col">
					<h1>{server.data.name}</h1>
					<p>{server.data.state}</p>
				</div>
				<div className="absolute right-0">
					<ServerStartStopButton serverId={slug.toString()} />
				</div>
			</div>

			<Tabs defaultValue="console" className="gap-2">
				<TabsList>
					<TabsTrigger value="console">Console</TabsTrigger>
					<TabsTrigger value="settings">Settings</TabsTrigger>
					<TabsTrigger value="backups">Backups</TabsTrigger>
				</TabsList>
				<TabsContent value="console">
					<ServerConsole serverId={slug.toString()} />
				</TabsContent>
				<TabsContent value="settings">
					<b>Settings</b>
				</TabsContent>
				<TabsContent value="backups">
					<b>Backups</b>
				</TabsContent>
			</Tabs>
		</PageLayout>
	);
}
