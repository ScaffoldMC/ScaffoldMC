"use client";

import {
	Avatar,
	AvatarFallback,
	AvatarImage,
} from "@/components/atoms/Avatar/Avatar";

import {
	HashTabs,
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
import { CloudBackup, Settings, Terminal } from "lucide-react";
import { ServerConfigForm } from "@/components/organisms/ServerConfigForm/ServerConfigForm";
import { ServerDangerActions } from "@/components/organisms/ServerDangerActions/ServerDangerActions";

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

	if (!server.data || server.error) {
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
					<h1>{server.data.config.name}</h1>
					<p>{server.data.state}</p>
				</div>
				<div className="absolute right-0">
					<ServerStartStopButton serverId={slug.toString()} />
				</div>
			</div>

			<HashTabs defaultValue="console" className="gap-2">
				<TabsList>
					<TabsTrigger value="console">
						<Terminal size={18} /> Console
					</TabsTrigger>
					<TabsTrigger value="settings">
						<Settings size={18} />
						Settings
					</TabsTrigger>
					<TabsTrigger value="backups">
						<CloudBackup size={18} />
						Backups
					</TabsTrigger>
				</TabsList>
				<TabsContent value="console">
					<ServerConsole serverId={slug.toString()} />
				</TabsContent>
				<TabsContent value="settings" className="flex flex-col gap-2">
					<h2>Configuration</h2>
					<ServerConfigForm serverId={slug.toString()} />
					<h2>Danger Zone</h2>
					<ServerDangerActions serverId={slug.toString()} />
				</TabsContent>
				<TabsContent value="backups">
					<b>Backups</b>
				</TabsContent>
			</HashTabs>
		</PageLayout>
	);
}
