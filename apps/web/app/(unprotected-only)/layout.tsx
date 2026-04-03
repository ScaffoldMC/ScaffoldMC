"use client";

import { useAuth } from "@/hooks/auth";
import { LoaderCircle } from "lucide-react";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function NoAuthLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	const { authenticated, isLoading } = useAuth();
	const router = useRouter();

	useEffect(() => {
		if (authenticated && !isLoading) {
			router.push("/home");
		}
	}, [authenticated, isLoading, router]);

	if (isLoading) {
		return (
			<div className="flex items-center justify-center min-h-screen">
				<LoaderCircle
					className="animate-spin text-text-primary"
					size={48}
				/>
			</div>
		);
	}

	if (authenticated && !isLoading) {
		return null;
	}

	return <>{children}</>;
}
