import type { Meta, StoryObj } from "@storybook/nextjs-vite";

import { PasswordResetPortal } from "./PasswordResetDialog";
import { DialogRoot } from "../Dialog/Dialog";

const meta = {
	render: (args) => (
		<DialogRoot open>
			<PasswordResetPortal {...args} />
		</DialogRoot>
	),
	component: PasswordResetPortal,
} satisfies Meta<typeof PasswordResetPortal>;
export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		onSubmit: async (oldPassword: string, newPassword: string) => {
			alert(`Old Password: ${oldPassword}, New Password: ${newPassword}`);
			return Promise.reject();
		},
	},
};
