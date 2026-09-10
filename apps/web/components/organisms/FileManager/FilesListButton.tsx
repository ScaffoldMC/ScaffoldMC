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
import { useServerFilesystem } from "@/hooks/serverFiles";
import { FormEvent, useEffect, useRef, useState } from "react";
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
				false: "hover:bg-accent-lightest dark:hover:bg-accent-darkest hover:text-text-primary",
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
	serverId: string;
}

export function FilesListButton({
	selected,
	onClick,
	fsEntry,
	serverId,
}: FilesListButtonProps) {
	const className = buttonStyles({ selected });
	const [renameMode, setRenameMode] = useState(false);
	const inputRef = useRef<HTMLInputElement>(null);
	const { renameEntry } = useServerFilesystem(serverId);

	useEffect(() => {
		if (!renameMode) {
			return;
		}

		const frame = requestAnimationFrame(() => {
			inputRef.current?.focus();
			inputRef.current?.select();
		});

		return () => cancelAnimationFrame(frame);
	}, [renameMode]);

	const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		const entryName = inputRef.current?.value.trim();

		if (!entryName || entryName === fsEntry.name) {
			setRenameMode(false);
			return;
		}

		await renameEntry({ path: fsEntry.path, to: entryName });
		setRenameMode(false);
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
									ref={inputRef}
									name="entryName"
									variant="ghost"
									type="text"
									defaultValue={fsEntry.name}
									onBlur={() => setRenameMode(false)}
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
					<ContextMenuContent
						onCloseAutoFocus={(event) => {
							if (renameMode) {
								event.preventDefault();
							}
						}}
					>
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
