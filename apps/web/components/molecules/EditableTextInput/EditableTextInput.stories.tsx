import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { EditableTextInput } from "./EditableTextInput";

const meta = {
	component: EditableTextInput,
} satisfies Meta<typeof EditableTextInput>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		value: "Some text",
		onChange: async (value) => {
			console.log(value);
		},
	},
};
