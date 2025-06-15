import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { Label } from "./Label";
import { TextInput } from "../TextInput/TextInput";
import { Checkbox } from "../Checkbox/Checkbox";

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
