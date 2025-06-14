import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Button } from "./button";

const meta = {
	component: Button,
} satisfies Meta<typeof Button>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Primary: Story = {
	render: (args) => <Button {...args}>Button</Button>,
	args: {
		level: "primary",
	},
};

export const Secondary: Story = {
	render: (args) => <Button {...args}>Button</Button>,
	args: {
		level: "secondary",
	},
};

export const Destructive: Story = {
	render: (args) => <Button {...args}>Button</Button>,
	args: {
		level: "destructive",
	},
};

export const Ghost: Story = {
	render: (args) => <Button {...args}>Button</Button>,
	args: {
		level: "ghost",
	},
};
