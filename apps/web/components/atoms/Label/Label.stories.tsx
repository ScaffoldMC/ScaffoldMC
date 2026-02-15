import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Label } from "./Label";

const meta = {
	component: Label,
} satisfies Meta<typeof Label>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		children: "Label",
	},
};
