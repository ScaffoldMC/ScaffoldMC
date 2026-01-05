import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Dropdown } from "./Dropdown";

const meta = {
	component: Dropdown,
	render: (args) => (
		<Dropdown {...args}>
			<option value="option1">Option 1</option>
			<label>Option 2</label>
			<option value="option2">Option 2</option>
			<option value="option3">Option 3</option>
		</Dropdown>
	),
} satisfies Meta<typeof Dropdown>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {},
};
