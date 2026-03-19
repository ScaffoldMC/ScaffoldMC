import { fileURLToPath } from "node:url";
import { dirname } from "node:path";
import type { StorybookConfig } from "@storybook/nextjs-vite";

const config: StorybookConfig = {
	stories: ["../**/*.@(mdx|stories.@(js|jsx|ts|tsx))"],
	addons: [getAbsolutePath("@storybook/addon-themes")],
	framework: {
		name: getAbsolutePath("@storybook/nextjs-vite"),
		options: {},
	},
};

export default config;

// To customize your webpack configuration you can use the webpackFinal field.
// Check https://storybook.js.org/docs/react/builders/webpack#extending-storybooks-webpack-config
// and https://nx.dev/recipes/storybook/custom-builder-configs

function getAbsolutePath(value: string): any {
	return dirname(fileURLToPath(import.meta.resolve(`${value}/package.json`)));
}
