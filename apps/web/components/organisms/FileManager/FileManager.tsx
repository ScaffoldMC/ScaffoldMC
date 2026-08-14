import { Button } from "@/components/atoms/Button/Button";
import { useServerFilesystem } from "@/hooks/serverFiles";
import { FSDirectoryEntry, FSEntry, FSFileEntry } from "@/lib/servertypes";
import { cn } from "@/lib/util";
import { FileIcon, FolderIcon, SaveIcon, UndoIcon } from "lucide-react";
import { createContext, useContext, useEffect, useState } from "react";
import { TextEditor, TextEditorMode } from "../TextEditor/TextEditor";

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

function FilePreview({
	serverId,
	file,
}: {
	serverId: string;
	file: FSFileEntry | null;
}) {
	const { readFile, writeFile } = useServerFilesystem(serverId);
	const [savedContent, setSavedContent] = useState<string | null>(null);
	const [editedContent, setEditedContent] = useState<string>("");
	const [unsavedChanges, setUnsavedChanges] = useState<boolean>(false);

	useEffect(() => {
		let mounted = true;
		if (!file) {
			setSavedContent(null);
			setEditedContent("");
			setUnsavedChanges(false);
			return;
		}

		readFile(file.path)
			.then((content) => {
				if (!mounted) return;
				setSavedContent(content ?? "");
				setEditedContent(content ?? "");
				setUnsavedChanges(false);
			})
			.catch(() => {
				if (!mounted) return;
				setSavedContent("");
				setEditedContent("");
				setUnsavedChanges(false);
			});

		return () => {
			mounted = false;
		};
	}, [file?.path]);

	const handleSave = async () => {
		if (!file) return;
		try {
			await writeFile({ path: file.path, content: editedContent });
			setSavedContent(editedContent);
			setUnsavedChanges(false);
		} catch (e) {
			// swallow for now; mutation will invalidate filesystem via hook
			console.error("Failed to save file", e);
		}
	};

	const handleChange = (value: string) => {
		setEditedContent(value);
		setUnsavedChanges(value !== (savedContent ?? ""));
	};

	const handleRestore = () => {
		setEditedContent(savedContent ?? "");
		setUnsavedChanges(false);
	};

	return (
		<div className="flex flex-col justify-center content-center rounded-md border border-border-static bg-surface overflow-clip">
			{file ? (
				<>
					<div className="border-border-static border-b h-8 flex flex-row items-center p-2 gap-1">
						<span className="font-mono text-sm">{file.path}</span>
						<Button
							size="variable"
							level="ghost"
							className="h-6"
							onClick={handleSave}
							hidden={!unsavedChanges}
						>
							<SaveIcon size={14} />
						</Button>
						<Button
							size="variable"
							level="ghost"
							className="h-6"
							onClick={handleRestore}
							hidden={!unsavedChanges}
						>
							<UndoIcon size={14} />
						</Button>
					</div>
					<TextEditor
						value={editedContent}
						onChange={handleChange}
						mode={TextEditorMode.JSON}
					/>
				</>
			) : (
				<div className="text-text-secondary flex flex-col items-center justify-center gap-2 p-4">
					<FileIcon size={24} />
					<p>Select a file to preview its contents</p>
				</div>
			)}
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
				<FilePreview file={selectedFile} serverId={serverId} />
			</div>
		</FileManagerContext>
	);
}
