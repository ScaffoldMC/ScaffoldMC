import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Button } from "./Button";
import { Globe } from "lucide-react";

const meta = {
	component: Button,
} satisfies Meta<typeof Button>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Primary: Story = {
	args: {
		level: "primary",
		children: "Button",
	},
};

export const Secondary: Story = {
	args: {
		level: "secondary",
		children: "Button",
	},
};

export const Destructive: Story = {
	args: {
		level: "destructive",
		children: "Button",
	},
};

export const Ghost: Story = {
	args: {
		level: "ghost",
		children: "Button",
	},
};

export const Icon: Story = {
	args: {
		level: "secondary",
		children: <Globe size={18} />,
	},
};

export const IconWithText: Story = {
	args: {
		level: "secondary",
		children: (
			<>
				<Globe size={18} />
				Button
			</>
		),
	},
};
