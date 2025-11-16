import { Button } from "@/components/atoms/Button/Button";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Check, Edit, X } from "lucide-react";
import styles from "./EditableTextInput.module.css";
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
		<div className={styles.container}>
			<TextInput
				disabled={!editMode}
				value={internalValue}
				onChange={(e) => setInternalValue(e.target.value)}
			/>
			{!editMode && editable && (
				<Button size="icon" onClick={() => setEditMode(true)}>
					<Edit size={18} />
				</Button>
			)}
			{editMode && (
				<>
					<Button size="icon" level="primary" onClick={handleConfirm}>
						<Check size={18} />
					</Button>
					<Button size="icon" onClick={handleCancel}>
						<X size={18} />
					</Button>
				</>
			)}
		</div>
	);
}
