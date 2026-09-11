import { Button } from "@/components/atoms/Button/Button";
import { useServerFilesystem } from "@/hooks/serverFiles";
import { FSDirectoryEntry, FSEntry, FSFileEntry } from "@/lib/servertypes";
import { useContext, useState } from "react";
import { FileManagerContext } from "./FileManager";
import { FilesListButton } from "./FilesListButton";
import { FilePlusCorner, FolderPlus } from "lucide-react";
import { getAvailableName } from "@/lib/util";

export function FilesList({
	files,
	serverId,
}: {
	files: FSEntry[];
	serverId: string;
}) {
	const { createFile, createDirectory, getMetadata } =
		useServerFilesystem(serverId);
	const { setSelectedFile } = useContext(FileManagerContext);

	const handleCreateFile = async () => {
		const path = getAvailableName(files, "New File");
		await createFile(path);

		const entry = await getMetadata(path);
		if (entry.type === "file") {
			setSelectedFile(entry);
		}
	};

	const handleCreateDirectory = () => {
		createDirectory(getAvailableName(files, "New Directory"));
	};

	return (
		<div className="flex min-w-0 flex-col rounded-md border border-border-static bg-surface">
			<div className="flex gap-1 p-1 pb-0">
				<Button level="ghost" onClick={handleCreateFile}>
					<FilePlusCorner size={18} />
				</Button>
				<Button level="ghost" onClick={handleCreateDirectory}>
					<FolderPlus size={18} />
				</Button>
			</div>
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
					<FileListing
						key={`${path}/${entry.name}`}
						file={entry}
						serverId={serverId}
					/>
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
			<FilesListButton
				onClick={handleClick}
				selected={selected}
				fsEntry={{ type: "dir", ...dir }}
				serverId={serverId}
			/>

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

function FileListing({
	file,
	serverId,
}: {
	file: FSFileEntry;
	serverId: string;
}) {
	const { selectedFile, setSelectedFile } = useContext(FileManagerContext);
	const selected = selectedFile?.name === file.name;

	const handleClick = () => {
		setSelectedFile(file);
	};

	return (
		<FilesListButton
			onClick={handleClick}
			selected={selected}
			fsEntry={{ type: "file", ...file }}
			serverId={serverId}
		/>
	);
}
