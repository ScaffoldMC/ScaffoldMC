import { Button } from "@/components/atoms/Button/Button";
import {
	ContextMenu,
	ContextMenuContent,
	ContextMenuItem,
	ContextMenuPortal,
	ContextMenuTrigger,
} from "@/components/molecules/ContextMenu/ContextMenu";
import { cn } from "@/lib/util";
import { cva } from "class-variance-authority";
import { SubmitEventHandler, useState } from "react";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { FSEntry } from "@/lib/servertypes";
import { FileIcon, FolderIcon } from "lucide-react";

const buttonStyles = cva(
	cn(
		"w-full text-sm text-text-secondary font-light",
		"inline-flex items-center justify-start cursor-pointer",
		"h-9 gap-2 rounded-md border px-2",
		"transition-[background-color,border-color,color] duration-100 ease-in-out",
		"[&_svg]:inline-block disabled:cursor-not-allowed disabled:opacity-50",
		"outline-none",
	),
	{
		variants: {
			selected: {
				true: cn(
					"text-text-primary bg-accent-lightest dark:bg-accent-darkest",
				),
				false: "hover:bg-accent-darkest hover:text-text-primary",
			},
		},
		defaultVariants: {
			selected: false,
		},
	},
);

export interface FilesListButtonProps {
	selected: boolean;
	onClick: () => void;
	fsEntry: FSEntry;
}

export function FilesListButton({
	selected,
	onClick,
	fsEntry,
}: FilesListButtonProps) {
	const className = buttonStyles({ selected });
	const [renameMode, setRenameMode] = useState(false);

	const handleSubmit: SubmitEventHandler = (event) => {
		event.preventDefault();
		const formData = new FormData(event.target);
		const entryName = formData.get("entryName");

		setRenameMode(false);
		console.log(entryName);
	};

	return (
		<>
			<ContextMenu>
				<ContextMenuTrigger asChild>
					{renameMode ? (
						<div className={className}>
							{fsEntry.type === "dir" ? (
								<FolderIcon size={18} />
							) : (
								<FileIcon size={18} />
							)}

							<form onSubmit={handleSubmit}>
								<TextInput
									name="entryName"
									variant="ghost"
									type="text"
									autoFocus
								/>
							</form>
						</div>
					) : (
						<Button
							level="ghost"
							className={className}
							onClick={onClick}
						>
							{fsEntry.type === "dir" ? (
								<FolderIcon size={18} />
							) : (
								<FileIcon size={18} />
							)}

							{fsEntry.name}
						</Button>
					)}
				</ContextMenuTrigger>
				<ContextMenuPortal>
					<ContextMenuContent>
						<ContextMenuItem onClick={() => setRenameMode(true)}>
							Rename
						</ContextMenuItem>
						<ContextMenuItem>Delete</ContextMenuItem>
					</ContextMenuContent>
				</ContextMenuPortal>
			</ContextMenu>
		</>
	);
}
