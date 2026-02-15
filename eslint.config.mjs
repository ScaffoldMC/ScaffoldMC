import pluginTypeScript from "@typescript-eslint/eslint-plugin";
import parserTypescript from "@typescript-eslint/parser";
import pluginNext from "@next/eslint-plugin-next";
import pluginReact from "eslint-plugin-react";
import pluginReactHooks from "eslint-plugin-react-hooks";
import pluginPrettier from "eslint-plugin-prettier";

import { defineConfig } from "eslint/config";

export default defineConfig([
	{
		ignores: ["**/node_modules/**", "**/.next/**"],
	},
	{
		ignores: ["apps/web/.storybook/**"],
		files: ["apps/web/**/*.{ts,tsx}"],
		languageOptions: {
			parser: parserTypescript,
			parserOptions: {
				projectService: true,
			},
		},
		plugins: {
			"@typescript-eslint": pluginTypeScript,
			"@next/next": pluginNext,
			react: pluginReact,
			"react-hooks": pluginReactHooks,
			prettier: pluginPrettier,
		},
		rules: {
			...pluginTypeScript.configs.recommended.rules,
			...pluginNext.configs.recommended.rules,
			...pluginReact.configs.recommended.rules,
			...pluginReactHooks.configs.recommended.rules,
			...pluginPrettier.configs.recommended.rules,
			"react/react-in-jsx-scope": "off",
			"@typescript-eslint/no-unused-vars": [
				"warn",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^_",
				},
			],
		},
		settings: {
			react: {
				version: "detect",
			},
			next: {
				rootDir: "apps/web",
			},
		},
	},
]);
