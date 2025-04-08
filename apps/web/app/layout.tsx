import type { Metadata } from "next";
import localFont from "next/font/local";
import "./globals.css";

export const metadata: Metadata = {
	title: "MC Server UI",
	description: "Minecraft Server UI",
};

const manropeFont = localFont({
	src: "../public/fonts/Manrope-VariableFont_wght.ttf",
});

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en" className={manropeFont.className}>
			<body>{children}</body>
		</html>
	);
}
