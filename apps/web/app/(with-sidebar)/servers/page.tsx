"use client";

import { Button } from "@/components/atoms/Button/Button";
import { ServerList } from "@/components/organisms/ServerList/ServerList";
import { useRouter } from "next/navigation";

export default function Servers() {
	const router = useRouter();

	const handleClick = () => {
		router.push("/create-server");
	};

	return (
		<div>
			<h1>Servers</h1>
			<Button onClick={handleClick}>Create Server</Button>
			<ServerList />
		</div>
	);
}
