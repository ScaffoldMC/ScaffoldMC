"use client";

import { InfoNugget } from "@/components/atoms/InfoNugget/InfoNugget";
import PageLayout from "@/components/atoms/PageLayout/PageLayout";
import { ServerListPreview } from "@/components/organisms/ServerListPreview/ServerListPreview";
import { useCurrentUser } from "@/hooks/user";

export default function Dashboard() {
	const { user } = useCurrentUser();

	return (
		<PageLayout>
			<h1>Hello {user.data?.fullname}</h1>
			<div className="flex flex-row gap-2 w-full bg-transparent">
				<InfoNugget title="Servers Online" value="2" />
				<InfoNugget title="Total Players" value="12" />
				<InfoNugget title="CPU Usage" value="33%" />
				<InfoNugget title="RAM Usage" value="4302 MB" />
				<InfoNugget title="Disk Usage" value="15 GB" />
			</div>

			<div className="flex flex-col gap-2 max-w-md">
				<h2>Recent Servers</h2>
				<ServerListPreview />
			</div>
		</PageLayout>
	);
}
