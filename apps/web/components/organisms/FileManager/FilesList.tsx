import { Button } from "@/components/atoms/Button/Button";
import { useServerFilesystem } from "@/hooks/serverFiles";
import { FSDirectoryEntry, FSEntry, FSFileEntry } from "@/lib/servertypes";
import { useContext, useState } from "react";
import { FileManagerContext } from "./FileManager";
import { DeleteEntryDialog } from "./DeleteEntryDialog";
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
	const { selectedFile, setSelectedFile } = useContext(FileManagerContext);
	const [entryToDelete, setEntryToDelete] = useState<FSEntry | null>(null);
	const [deleting, setDeleting] = useState(false);
	const { deleteEntry } = useServerFilesystem(serverId);

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

	const requestDelete = (entry: FSEntry) => {
		requestAnimationFrame(() => setEntryToDelete(entry));
	};

	const handleDelete = async () => {
		if (!entryToDelete) return;

		const deletedPath = entryToDelete.path.replace(/^\/+|\/+$/g, "");
		const selectedPath = selectedFile?.path.replace(/^\/+|\/+$/g, "");

		if (
			selectedPath === deletedPath ||
			selectedPath?.startsWith(`${deletedPath}/`)
		) {
			setSelectedFile(null);
		}

		setDeleting(true);

		try {
			await deleteEntry(entryToDelete.path);
		} finally {
			setDeleting(false);
			setEntryToDelete(null);
		}
	};

	return (
		<div className="flex min-w-0 flex-col rounded-md border border-border-static bg-surface">
			<DeleteEntryDialog
				entry={entryToDelete}
				deleting={deleting}
				onCancel={() => setEntryToDelete(null)}
				onConfirm={handleDelete}
			/>
			<div className="flex gap-1 p-1 pb-0">
				<Button level="ghost" onClick={handleCreateFile}>
					<FilePlusCorner size={18} />
				</Button>
				<Button level="ghost" onClick={handleCreateDirectory}>
					<FolderPlus size={18} />
				</Button>
			</div>
			<FileTree
				files={files}
				serverId={serverId}
				onDelete={requestDelete}
			/>
		</div>
	);
}

function FileTree({
	files,
	serverId,
	onDelete,
	path = "",
}: {
	files: FSEntry[];
	serverId: string;
	onDelete: (entry: FSEntry) => void;
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
							onDelete={onDelete}
							path={path}
						/>
					);
				}

				return (
					<FileListing
						key={`${path}/${entry.name}`}
						file={entry}
						serverId={serverId}
						onDelete={onDelete}
					/>
				);
			})}
		</ul>
	);
}

function DirectoryListing({
	dir,
	serverId,
	onDelete,
	path = "",
}: {
	dir: FSDirectoryEntry;
	serverId: string;
	onDelete: (entry: FSEntry) => void;
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
				onDelete={onDelete}
				fsEntry={{ type: "dir", ...dir }}
				serverId={serverId}
			/>

			<div className="flex" hidden={!open}>
				<div className="bg-border-static rounded-full w-0.5 ml-2 my-1" />
				<FileTree
					files={content ?? []}
					serverId={serverId}
					onDelete={onDelete}
					path={nextPath}
				/>
			</div>
		</>
	);
}

function FileListing({
	file,
	serverId,
	onDelete,
}: {
	file: FSFileEntry;
	serverId: string;
	onDelete: (entry: FSEntry) => void;
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
			onDelete={onDelete}
			fsEntry={{ type: "file", ...file }}
			serverId={serverId}
		/>
	);
}
