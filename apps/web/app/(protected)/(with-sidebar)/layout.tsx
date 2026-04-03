import {
	Sidebar,
	SidebarContent,
	SidebarHeader,
	SidebarFooter,
	SidebarLink,
} from "@/components/organisms/Sidebar/Sidebar";
import { User } from "@/components/molecules/User/User";
import { Home, Server, Settings } from "lucide-react";
import Image from "next/image";

export default async function DashboardLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<div className="flex flex-row">
			<Sidebar>
				<SidebarHeader>
					<Image
						src="/images/logo-light.svg"
						alt="ScaffoldMC Logo"
						width={300}
						height={64}
						className="block dark:hidden"
					/>
					<Image
						src="/images/logo-dark.svg"
						alt="ScaffoldMC Logo"
						width={300}
						height={64}
						className="hidden dark:block"
					/>
				</SidebarHeader>
				<SidebarContent>
					<SidebarLink href="/home">
						<Home size={18} />
						Dashboard
					</SidebarLink>
					<SidebarLink href="/servers">
						<Server size={18} />
						Servers
					</SidebarLink>
					<SidebarLink href="/settings">
						<Settings size={18} />
						Settings
					</SidebarLink>
				</SidebarContent>
				<SidebarFooter>
					<User />
				</SidebarFooter>
			</Sidebar>
			<div className="flex flex-col flex-1 m-0 p-8 h-screen overflow-auto">
				{children}
			</div>
		</div>
	);
}
