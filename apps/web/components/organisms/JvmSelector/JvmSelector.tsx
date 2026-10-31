import { useQuery } from "@tanstack/react-query";
import { ChangeEventHandler, useEffect, useRef, useState } from "react";
import { Dropdown } from "@/components/atoms/Dropdown/Dropdown";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import api from "@/lib/axios";

interface Jvm {
	major_version: number;
	version_string: string;
	vendor: string;
	arch: string;
	path: string;
}

const CUSTOM_VALUE = "__custom__";

export function JvmSelector({
	name,
	value,
	onChange,
	onBlur,
	ref,
}: {
	name?: string;
	value?: string;
	onChange: (value: string) => void;
	onBlur: () => void;
	ref?: React.Ref<HTMLSelectElement>;
}) {
	const jvms = useQuery<Jvm[]>({
		queryKey: ["jvms"],
		queryFn: async () => {
			const response = await api.get("/jvms");
			return response.data;
		},
	});

	const selectedPath = value ?? "";
	const hasSelectedJvm = jvms.data?.some((jvm) => jvm.path === selectedPath);
	const [customSelected, setCustomSelected] = useState(false);
	const customSelectionRef = useRef(false);

	useEffect(() => {
		if (customSelectionRef.current) {
			customSelectionRef.current = false;
			return;
		}

		setCustomSelected(Boolean(selectedPath && !hasSelectedJvm));
	}, [selectedPath, hasSelectedJvm]);

	const handleSelectChange: ChangeEventHandler<HTMLSelectElement> = (
		event,
	) => {
		if (event.target.value === CUSTOM_VALUE) {
			customSelectionRef.current = true;
			setCustomSelected(true);
			onChange("");
		} else {
			setCustomSelected(false);
			onChange(event.target.value);
		}
		onBlur();
	};

	if (jvms.isError) {
		return (
			<p className="text-sm text-text-secondary">
				Unable to load installed Java runtimes.
			</p>
		);
	}

	return (
		<div className="flex min-w-0 flex-col gap-1">
			<Dropdown
				ref={ref}
				className="w-full max-w-xl"
				name={name}
				value={customSelected ? CUSTOM_VALUE : selectedPath}
				onChange={handleSelectChange}
				onBlur={onBlur}
				disabled={jvms.isLoading}
				aria-busy={jvms.isLoading}
			>
				<option value="">
					{jvms.isLoading
						? "Loading Java runtimes..."
						: !jvms.data?.length
							? "No Java runtimes found"
							: "Select a Java runtime"}
				</option>
				{jvms.data?.map((jvm) => (
					<option key={jvm.path} value={jvm.path}>
						Java {jvm.major_version} - {jvm.version_string} -{" "}
						{jvm.vendor} ({jvm.arch})
					</option>
				))}
				<option value={CUSTOM_VALUE}>Custom</option>
			</Dropdown>
			{customSelected && (
				<TextInput
					name={`${name}-custom`}
					value={selectedPath}
					placeholder="Path to Java executable"
					aria-label="Custom Java path"
					className="w-full max-w-xl"
					onChange={(event) => onChange(event.target.value)}
					onBlur={onBlur}
				/>
			)}
			{selectedPath && !customSelected && (
				<span className="break-all text-xs text-text-secondary">
					{selectedPath}
				</span>
			)}
		</div>
	);
}
