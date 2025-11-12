import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { PasswordDialog } from "./PasswordDialog";

const meta = {
	component: PasswordDialog,
} satisfies Meta<typeof PasswordDialog>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		onPassword: async (password: string) => {
			alert(`Password entered: ${password}`);
			return Promise.reject();
		},
	},
};
