import { Navbar, NavbarLink } from "@/components/organisms/Navbar/Navbar";
import { Server, Settings } from "lucide-react";

export default async function DashboardLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<div className="flex flex-col">
			<Navbar>
				<NavbarLink href="/servers">
					<Server size={22} />
					Servers
				</NavbarLink>
				<NavbarLink href="/settings">
					<Settings size={22} />
					Settings
				</NavbarLink>
			</Navbar>
			<div className="flex flex-col flex-1 m-0 p-8 h-screen overflow-auto">
				{children}
			</div>
		</div>
	);
}
