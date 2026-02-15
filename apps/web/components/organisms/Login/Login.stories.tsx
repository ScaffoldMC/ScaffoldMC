import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Login } from "./Login";

const meta = {
	component: Login,
} satisfies Meta<typeof Login>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		onLogin: (_username, _password) => {
			return Promise.resolve();
		},
	},
};

export const Invalid: Story = {
	args: {
		onLogin: (_username, _password) => {
			return Promise.reject();
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = canvasElement as HTMLCanvasElement;
		const usernameInput = canvas.querySelector(
			'input[name="username"]',
		) as HTMLInputElement;
		const passwordInput = canvas.querySelector(
			'input[name="password"]',
		) as HTMLInputElement;
		const submitButton = canvas.querySelector(
			'button[type="submit"]',
		) as HTMLButtonElement;

		usernameInput.value = "invalidUser";
		passwordInput.value = "invalidPass";

		submitButton.click();
	},
};
