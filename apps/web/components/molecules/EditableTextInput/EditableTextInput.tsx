import { Button } from "@/components/atoms/Button/Button";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Check, Edit, X } from "lucide-react";
import { useState } from "react";

interface EditableTextInputProps {
	editable?: boolean;
	value?: string;
	onChange: (value: string) => Promise<void>;
}

export function EditableTextInput({
	editable = true,
	value,
	onChange,
}: EditableTextInputProps) {
	const [internalValue, setInternalValue] = useState(value || "");
	const [editMode, setEditMode] = useState(false);

	const handleConfirm = () => {
		onChange(internalValue).then(() => {
			setInternalValue(internalValue);
		});
		setEditMode(false);
	};

	const handleCancel = () => {
		setInternalValue(value || "");
		setEditMode(false);
	};

	return (
		<div className="flex flex-row gap-1">
			<TextInput
				style={{
					minWidth: 0,
					flexGrow: 1,
					flexShrink: 1,
				}}
				disabled={!editMode}
				value={internalValue}
				onChange={(e) => setInternalValue(e.target.value)}
			/>
			{!editMode && editable && (
				<Button onClick={() => setEditMode(true)}>
					<Edit size={18} />
				</Button>
			)}
			{editMode && (
				<>
					<Button level="primary" onClick={handleConfirm}>
						<Check size={18} />
					</Button>
					<Button onClick={handleCancel}>
						<X size={18} />
					</Button>
				</>
			)}
		</div>
	);
}
