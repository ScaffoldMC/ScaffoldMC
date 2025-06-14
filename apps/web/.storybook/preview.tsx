import type { Preview } from "@storybook/nextjs-vite";
import localFont from "next/font/local";

import "@/app/globals.css";

const manropeFont = localFont({
	src: "../public/fonts/Manrope-VariableFont_wght.ttf",
	variable: "--font-manrope",
});

const preview: Preview = {
	parameters: {},
	decorators: [
		(Story) => (
			<div className={manropeFont.className}>
				<Story />
			</div>
		),
	],
};

export default preview;
