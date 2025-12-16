import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { PasswordDialogPortal } from "./PasswordDialog";
import { DialogRoot } from "../Dialog/Dialog";

const meta = {
	render: (args) => (
		<DialogRoot open>
			<PasswordDialogPortal {...args} />
		</DialogRoot>
	),
	component: PasswordDialogPortal,
} satisfies Meta<typeof PasswordDialogPortal>;
export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		onPassword: async (password: string) => {
			alert(`Password entered: ${password}`);
			return Promise.reject();
		},
	},
};
