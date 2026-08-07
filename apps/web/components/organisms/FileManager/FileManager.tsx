import { Button } from "@/components/atoms/Button/Button";
import { useServerFilesystem } from "@/hooks/serverFiles";
import { FSDirectoryEntry, FSEntry, FSFileEntry } from "@/lib/servertypes";
import { cn } from "@/lib/util";
import { FileIcon, FolderIcon } from "lucide-react";
import { createContext, useContext, useState } from "react";

interface FileManagerContextValue {
	selectedFile: FSFileEntry | null;
	setSelectedFile: (file: FSFileEntry | null) => void;
}

const FileManagerContext = createContext<FileManagerContextValue>({
	selectedFile: null,
	setSelectedFile: () => {},
});

function FilesList({
	files,
	serverId,
}: {
	files: FSEntry[];
	serverId: string;
}) {
	return (
		<div className="flex min-w-0 flex-col rounded-md border border-border-static bg-surface">
			<FileTree files={files} serverId={serverId} />
		</div>
	);
}

function FileTree({
	files,
	serverId,
	path = "",
}: {
	files: FSEntry[];
	serverId: string;
	path?: string;
}) {
	return (
		<ul className="flex-1 overflow-y-scroll p-1">
			{files.map((entry) => {
				if (entry.type === "dir") {
					return (
						<DirectoryListing
							key={`${path}/${entry.name}`}
							dir={entry}
							serverId={serverId}
							path={path}
						/>
					);
				}

				return (
					<FileListing key={`${path}/${entry.name}`} file={entry} />
				);
			})}
		</ul>
	);
}

function DirectoryListing({
	dir,
	serverId,
	path = "",
}: {
	dir: FSDirectoryEntry;
	serverId: string;
	path?: string;
}) {
	const [open, setOpen] = useState(false);
	const [content, setContent] = useState<FSEntry[] | null>(null);
	const { listDirectory } = useServerFilesystem(serverId);

	const { selectedFile } = useContext(FileManagerContext);
	const selected = selectedFile?.name === dir.name;
	const nextPath = path ? `${path}/${dir.name}` : dir.name;

	const handleClick = async () => {
		if (!content) {
			const entries = await listDirectory(nextPath);
			setContent(entries);
		}

		setOpen((current) => !current);
	};

	return (
		<>
			<Button
				level="ghost"
				className={cn(
					"w-full justify-start text-sm text-text-secondary font-light hover:bg-accent-darkest hover:text-text-primary",
					selected && "text-text-primary bg-surface-raised",
				)}
				onClick={handleClick}
			>
				<FolderIcon size={18} />
				{dir.name}
			</Button>
			<div className="flex" hidden={!open}>
				<div className="bg-border-static rounded-full w-0.5 ml-2 my-1" />
				<FileTree
					files={content ?? []}
					serverId={serverId}
					path={nextPath}
				/>
			</div>
		</>
	);
}

function FileListing({ file }: { file: FSFileEntry }) {
	const { selectedFile, setSelectedFile } = useContext(FileManagerContext);
	const selected = selectedFile?.name === file.name;

	const handleClick = () => {
		setSelectedFile(file);
	};

	return (
		<Button
			level="ghost"
			className={cn(
				"w-full justify-start text-sm text-text-secondary font-light hover:bg-accent-darkest hover:text-text-primary",
				selected && "text-text-primary bg-accent-lightest",
			)}
			onClick={handleClick}
		>
			<FileIcon size={18} />
			{file.name}
		</Button>
	);
}

function FilePreview({ file }: { file: FSFileEntry }) {
	return (
		<div className="flex flex-col justify-center content-center rounded-md border border-border-static bg-surface">
			<div className="text-text-secondary flex flex-col items-center justify-center gap-2 p-4">
				<FileIcon size={24} />
				<p>Select a file to preview its contents</p>
			</div>
		</div>
	);
}

export function FileManager({ serverId }: { serverId: string }) {
	const { filesystem } = useServerFilesystem(serverId);
	const [selectedFile, setSelectedFile] = useState<FSFileEntry | null>(null);

	return (
		<FileManagerContext value={{ selectedFile, setSelectedFile }}>
			<div className="grid h-128 max-h-128 w-full grid-cols-[18rem_1fr] gap-2">
				<FilesList files={filesystem.data ?? []} serverId={serverId} />
				<FilePreview file={selectedFile} />
			</div>
		</FileManagerContext>
	);
}
