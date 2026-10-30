import { useServerFilesystem } from "@/hooks/serverFiles";
import { FSFileEntry } from "@/lib/servertypes";
import { createContext, useState } from "react";
import { FilesList } from "./FilesList";
import { FilePreview } from "./FilePreview";

interface FileManagerContextValue {
	selectedFile: FSFileEntry | null;
	setSelectedFile: (file: FSFileEntry | null) => void;
}

export const FileManagerContext = createContext<FileManagerContextValue>({
	selectedFile: null,
	setSelectedFile: () => {},
});

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
