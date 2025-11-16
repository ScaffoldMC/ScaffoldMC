import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Alert } from "./Alert";

const meta = {
	component: Alert,
} satisfies Meta<typeof Alert>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Info: Story = {
	args: {
		type: "info",
		children: (
			<>
				<h3>Hello</h3>
				<p>hello hello hello</p>
			</>
		),
	},
};

export const Warning: Story = {
	args: {
		type: "warning",
		children: (
			<>
				<h3>Hello</h3>
				<p>hello hello hello</p>
			</>
		),
	},
};

export const Error: Story = {
	args: {
		type: "error",
		children: (
			<>
				<h3>Hello</h3>
				<p>hello hello hello</p>
			</>
		),
	},
};
