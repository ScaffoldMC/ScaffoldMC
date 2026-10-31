import { useQuery } from "@tanstack/react-query";
import { Dropdown } from "@/components/atoms/Dropdown/Dropdown";
import api from "@/lib/axios";

interface Jvm {
	major_version: number;
	version_string: string;
	vendor: string;
	arch: string;
	path: string;
}

export function JvmSelector({
	name,
	value,
	onChange,
	onBlur,
	ref,
}: {
	name?: string;
	value?: string;
	onChange: React.ChangeEventHandler<HTMLSelectElement>;
	onBlur: React.FocusEventHandler<HTMLSelectElement>;
	ref?: React.Ref<HTMLSelectElement>;
}) {
	const jvms = useQuery<Jvm[]>({
		queryKey: ["jvms"],
		queryFn: async () => {
			const response = await api.get("/jvms");
			return response.data;
		},
	});

	if (jvms.isError) {
		return (
			<p className="text-sm text-text-secondary">
				Unable to load installed Java runtimes.
			</p>
		);
	}

	const selectedPath = value ?? "";
	const hasSelectedJvm = jvms.data?.some((jvm) => jvm.path === selectedPath);

	return (
		<div className="flex min-w-0 flex-col gap-1">
			<Dropdown
				ref={ref}
				className="w-full max-w-xl"
				name={name}
				value={selectedPath}
				onChange={onChange}
				onBlur={onBlur}
				disabled={jvms.isLoading || !jvms.data?.length}
				aria-busy={jvms.isLoading}
			>
				<option value="">
					{jvms.isLoading
						? "Loading Java runtimes..."
						: !jvms.data?.length
							? "No Java runtimes found"
							: "Select a Java runtime"}
				</option>
				{selectedPath && !hasSelectedJvm && (
					<option value={selectedPath}>Current Java runtime</option>
				)}
				{jvms.data?.map((jvm) => (
					<option key={jvm.path} value={jvm.path}>
						Java {jvm.major_version} - {jvm.version_string} -{" "}
						{jvm.vendor} ({jvm.arch})
					</option>
				))}
			</Dropdown>
			{selectedPath && (
				<span className="break-all text-xs text-text-secondary">
					{selectedPath}
				</span>
			)}
		</div>
	);
}
