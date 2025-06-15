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

import styles from "./page.module.css";

export default async function Page({
	params,
}: {
	params: Promise<{ slug: string }>;
}) {
	const { slug } = await params;
	return (
		<div className={styles.page}>
			<div className={styles.title}>
				<Avatar size={64} shape="square-medium">
					<AvatarFallback>?</AvatarFallback>
					<AvatarImage src="/images/server-default.png" />
				</Avatar>
				<div className={styles.titleInfo}>
					<h1>Server Name</h1>
					<p>Additional info</p>
				</div>
			</div>

			<Tabs defaultValue="console">
				<TabsList>
					<TabsTrigger value="console">Console</TabsTrigger>
					<TabsTrigger value="settings">Settings</TabsTrigger>
					<TabsTrigger value="backups">Backups</TabsTrigger>
				</TabsList>
				<TabsContent value="console">
					<b>Console</b>
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
