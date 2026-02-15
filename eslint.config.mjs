import pluginTypeScript from "@typescript-eslint/eslint-plugin";
import parserTypescript from "@typescript-eslint/parser";
import pluginNext from "@next/eslint-plugin-next";
import pluginReact from "eslint-plugin-react";
import pluginReactHooks from "eslint-plugin-react-hooks";
import pluginPrettier from "eslint-plugin-prettier";
import pluginImport from "eslint-plugin-import";
import path from "node:path";

import { defineConfig } from "eslint/config";

const __dirname = path.dirname(new URL(import.meta.url).pathname);

export default defineConfig([
	{
		ignores: ["**/node_modules/**", "**/.next/**"],
	},
	{
		ignores: ["apps/web/.storybook/**", "apps/web/lib/servertypes.ts"],
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
			import: pluginImport,
		},
		rules: {
			...pluginTypeScript.configs.recommended.rules,
			...pluginNext.configs.recommended.rules,
			...pluginReact.configs.recommended.rules,
			...pluginReactHooks.configs.recommended.rules,
			...pluginPrettier.configs.recommended.rules,
			...pluginImport.configs.recommended.rules,
			"react/react-in-jsx-scope": "off",
			"@typescript-eslint/no-unused-vars": [
				"warn",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^_",
				},
			],
			"n/no-missing-import": "off",
		},
		settings: {
			react: {
				version: "detect",
			},
			next: {
				rootDir: path.join(__dirname, "apps/web"),
			},
			"import/resolver": {
				typescript: {
					project: path.join(__dirname, "apps/web/tsconfig.json"),
				},
			},
		},
	},
]);
