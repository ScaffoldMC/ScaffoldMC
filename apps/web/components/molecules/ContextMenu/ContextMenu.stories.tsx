import type { Meta, StoryObj } from "@storybook/nextjs-vite";
import { useState } from "react";
import { Check, ChevronRight, Circle } from "lucide-react";

import {
	ContextMenu,
	ContextMenuCheckboxItem,
	ContextMenuContent,
	ContextMenuItem,
	ContextMenuLabel,
	ContextMenuItemIndicator,
	ContextMenuPortal,
	ContextMenuRadioGroup,
	ContextMenuRadioItem,
	ContextMenuSeparator,
	ContextMenuTrigger,
	ContextMenuSubContent,
	ContextMenuSub,
	ContextMenuSubTrigger,
} from "./ContextMenu";

import { cn } from "@/lib/util";

const meta = {
	component: ContextMenu,
} satisfies Meta<typeof ContextMenu>;

export default meta;

type Story = StoryObj<typeof meta>;

function ComprehensiveMenu() {
	const [showStatusBar, setShowStatusBar] = useState(true);
	const [person, setPerson] = useState("pedro");

	return (
		<ContextMenu>
			<ContextMenuTrigger
				className={cn(
					"flex min-h-48 w-full items-center justify-center rounded-lg border border-dashed border-border-static p-6 text-text-secondary",
				)}
			>
				Right-click this area to open the context menu
			</ContextMenuTrigger>
			<ContextMenuPortal>
				<ContextMenuContent>
					<ContextMenuLabel>Actions</ContextMenuLabel>
					<ContextMenuItem
						onSelect={() => console.info("Back selected")}
					>
						Back
					</ContextMenuItem>
					<ContextMenuItem
						onSelect={() => console.info("Forward selected")}
					>
						Forward
					</ContextMenuItem>
					<ContextMenuItem disabled>
						Reload (disabled)
					</ContextMenuItem>
					<ContextMenuSeparator />
					<ContextMenuLabel>View options</ContextMenuLabel>
					<ContextMenuCheckboxItem
						checked={showStatusBar}
						onCheckedChange={setShowStatusBar}
					>
						<ContextMenuItemIndicator>
							<Check size={14} />
						</ContextMenuItemIndicator>
						Show status bar
					</ContextMenuCheckboxItem>
					<ContextMenuCheckboxItem disabled checked>
						<ContextMenuItemIndicator>
							<Check size={14} />
						</ContextMenuItemIndicator>
						Always disabled
					</ContextMenuCheckboxItem>
					<ContextMenuSeparator />
					<ContextMenuLabel>People</ContextMenuLabel>
					<ContextMenuRadioGroup
						value={person}
						onValueChange={setPerson}
					>
						<ContextMenuRadioItem value="Alex">
							<ContextMenuItemIndicator>
								<Circle size={8} fill="currentColor" />
							</ContextMenuItemIndicator>
							Alex
						</ContextMenuRadioItem>
						<ContextMenuRadioItem value="Steve">
							<ContextMenuItemIndicator>
								<Circle size={8} fill="currentColor" />
							</ContextMenuItemIndicator>
							Steve
						</ContextMenuRadioItem>
						<ContextMenuRadioItem disabled value="disabled">
							<ContextMenuItemIndicator>
								<Circle size={8} fill="currentColor" />
							</ContextMenuItemIndicator>
							James
						</ContextMenuRadioItem>
					</ContextMenuRadioGroup>
					<ContextMenuSeparator />
					<ContextMenuSub>
						<ContextMenuSubTrigger>
							More tools
							<ChevronRight className="ml-auto" size={14} />
						</ContextMenuSubTrigger>
						<ContextMenuSubContent>
							<ContextMenuItem>Save as...</ContextMenuItem>
							<ContextMenuItem>Export</ContextMenuItem>
							<ContextMenuItem disabled>
								Print (disabled)
							</ContextMenuItem>
						</ContextMenuSubContent>
					</ContextMenuSub>
				</ContextMenuContent>
			</ContextMenuPortal>
		</ContextMenu>
	);
}

export const Default: Story = {
	render: () => <ComprehensiveMenu />,
};
