import { Button } from "@/components/atoms/Button/Button";
import { useServerFilesystem } from "@/hooks/serverFiles";
import { FSDirectoryEntry, FSEntry, FSFileEntry } from "@/lib/servertypes";
import { cn } from "@/lib/util";
import { FileIcon, FolderIcon } from "lucide-react";
import { useContext, useState } from "react";
import { FileManagerContext } from "./FileManager";
import { FilesListButton } from "./FilesListButton";

export function FilesList({
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
			<FilesListButton onClick={handleClick} selected={selected}>
				<FolderIcon size={18} />
				{dir.name}
			</FilesListButton>
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
		<FilesListButton onClick={handleClick} selected={selected}>
			<FileIcon size={18} />
			{file.name}
		</FilesListButton>
	);
}
