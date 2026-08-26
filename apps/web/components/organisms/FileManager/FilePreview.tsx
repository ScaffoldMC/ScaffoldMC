import { Button } from "@/components/atoms/Button/Button";
import { useServerFilesystem } from "@/hooks/serverFiles";
import { FSFileEntry } from "@/lib/servertypes";
import { FileIcon, SaveIcon, UndoIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { TextEditor, TextEditorMode } from "../TextEditor/TextEditor";

export function FilePreview({
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
