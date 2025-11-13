import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { PasswordDialog, PasswordDialogPortal } from "./PasswordDialog";

const meta = {
	render: (args) => (
		<PasswordDialog open>
			<PasswordDialogPortal {...args} />
		</PasswordDialog>
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
