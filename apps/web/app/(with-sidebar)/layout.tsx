import {
	Sidebar,
	SidebarContent,
	SidebarHeader,
	SidebarFooter,
	SidebarLink,
} from "@/components/organisms/Sidebar/Sidebar";
import { User } from "@/components/molecules/User/User";
import { Home, PlusCircle, Server, Settings } from "lucide-react";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import Image from "next/image";

export default async function DashboardLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	// Redirect if the user doesn't have a refresh token.
	// This isn't an actual auth check, but a cleaner way to redirect without a
	// flash of web content.

	const refresh_token = (await cookies()).get("ref_token")?.value;

	if (!refresh_token) {
		redirect("/login");
	}

	return (
		<div className="flex flex-row">
			<Sidebar>
				<SidebarHeader>
					<Image
						src="/images/logo.svg"
						alt="ScaffoldMC Logo"
						width={300}
						height={64}
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
