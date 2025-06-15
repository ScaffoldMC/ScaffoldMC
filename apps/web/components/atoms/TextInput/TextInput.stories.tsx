import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { TextInput } from "./TextInput";

const meta = {
	component: TextInput,
} satisfies Meta<typeof TextInput>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		placeholder: "Enter text",
	},
};

export const Invalid: Story = {
	args: {
		placeholder: "Password",
		invalid: true,
	},
};
