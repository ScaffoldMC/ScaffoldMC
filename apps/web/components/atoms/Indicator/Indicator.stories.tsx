import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Indicator } from "./Indicator";

const meta = {
	component: Indicator,
} satisfies Meta<typeof Indicator>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Success: Story = {
	args: {
		state: "success",
	},
};

export const Error: Story = {
	args: {
		state: "error",
	},
};

export const Working: Story = {
	args: {
		state: "working",
	},
};
