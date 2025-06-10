import pluginNext from "@next/eslint-plugin-next";
import pluginReact from "eslint-plugin-react";
import pluginReactHooks from "eslint-plugin-react-hooks";
import pluginPrettier from "eslint-plugin-prettier";

import { defineConfig } from "eslint/config";

export default defineConfig([
	{
		plugins: {
			"@next/next": pluginNext,
			react: pluginReact,
			"react-hooks": pluginReactHooks,
			prettier: pluginPrettier,
		},
		rules: {
			...pluginNext.configs.recommended.rules,
			...pluginReact.configs.recommended.rules,
			...pluginReactHooks.configs.recommended.rules,
			...pluginPrettier.configs.recommended.rules,
		},
	},
]);
