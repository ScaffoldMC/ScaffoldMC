import AceEditor from "react-ace";
import "./TextEditor.ace.css";

/* eslint-disable import/no-unresolved */
import "ace-builds/src-noconflict/ace";
import "ace-builds/src-noconflict/mode-json";
import "ace-builds/src-noconflict/mode-properties";
import "ace-builds/src-noconflict/mode-yaml";
import "ace-builds/src-noconflict/mode-toml";
import "ace-builds/src-noconflict/ext-language_tools";
/* eslint-enable import/no-unresolved */

export enum TextEditorMode {
	JSON = "json",
	PROPERTIES = "properties",
	YAML = "yaml",
	TOML = "toml",
}

const ACE_MODE_BY_EDITOR_MODE: Record<TextEditorMode, string> = {
	[TextEditorMode.JSON]: "json",
	[TextEditorMode.PROPERTIES]: "properties",
	[TextEditorMode.YAML]: "yaml",
	[TextEditorMode.TOML]: "toml",
};

export interface TextEditorProps {
	value: string;
	onChange?: (value: string) => void;
	readOnly?: boolean;
	mode?: TextEditorMode;
}

export function TextEditor({
	value,
	onChange,
	readOnly,
	mode = TextEditorMode.JSON,
}: TextEditorProps) {
	return (
		<AceEditor
			value={value}
			onChange={onChange}
			readOnly={readOnly}
			mode={ACE_MODE_BY_EDITOR_MODE[mode]}
			width="100%"
			height="100%"
			className="ace-scaffoldmc"
		/>
	);
}
