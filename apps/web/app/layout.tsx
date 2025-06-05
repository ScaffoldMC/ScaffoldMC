import type { Metadata } from "next";
import localFont from "next/font/local";
import "./globals.css";
import Providers from "@/lib/providers";

export const metadata: Metadata = {
	title: "MC Server UI",
	description: "Minecraft Server UI",
};

const manropeFont = localFont({
	src: "../public/fonts/Manrope-VariableFont_wght.ttf",
	variable: "--font-manrope",
});

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en">
			<body className={manropeFont.className}>
				<Providers>{children}</Providers>
			</body>
		</html>
	);
}
