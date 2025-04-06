import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
	title: "MC Server UI",
	description: "Minecraft Server UI",
};

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en">
			<body>{children}</body>
		</html>
	);
}
