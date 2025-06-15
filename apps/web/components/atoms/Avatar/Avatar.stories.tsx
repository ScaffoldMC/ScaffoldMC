import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Avatar, AvatarFallback } from "./Avatar";

const meta = {
	component: Avatar,
} satisfies Meta<typeof Avatar>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Placeholder: Story = {
	render: (args) => (
		<Avatar {...args}>
			<AvatarFallback>AZ</AvatarFallback>
		</Avatar>
	),
	args: {
		size: 64,
	},
};
