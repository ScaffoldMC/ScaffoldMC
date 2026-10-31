import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Button } from "@/components/atoms/Button/Button";
import { Dropdown } from "@/components/atoms/Dropdown/Dropdown";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import {
	DialogContent,
	DialogOverlay,
	DialogPortal,
	DialogRoot,
} from "@/components/organisms/Dialog/Dialog";
import api from "@/lib/axios";
import { XIcon } from "lucide-react";

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
	onChange: (value: string) => void;
	onBlur: () => void;
	ref?: React.Ref<HTMLInputElement>;
}) {
	const [selectorOpen, setSelectorOpen] = useState(false);
	const [detectedPath, setDetectedPath] = useState("");
	const jvms = useQuery<Jvm[]>({
		queryKey: ["jvms"],
		queryFn: async () => {
			const response = await api.get("/jvms");
			return response.data;
		},
	});

	const selectedPath = value ?? "";

	return (
		<>
			<div className="flex min-w-0 flex-row gap-2">
				<TextInput
					ref={ref}
					name={name}
					value={selectedPath}
					placeholder="Path to Java executable"
					className="w-full max-w-xl"
					onChange={(event) => onChange(event.target.value)}
					onBlur={onBlur}
				/>
				<Button
					type="button"
					level="secondary"
					size="default"
					onClick={() => {
						setDetectedPath("");
						setSelectorOpen(true);
					}}
				>
					Detect
				</Button>
			</div>
			<DialogRoot open={selectorOpen} onOpenChange={setSelectorOpen}>
				<DialogPortal>
					<DialogOverlay />
					<DialogContent className="items-stretch gap-4 p-6">
						<div className="flex items-center justify-between">
							<DialogPrimitive.Title>
								Select JVM
							</DialogPrimitive.Title>
							<DialogPrimitive.Close asChild>
								<Button type="button" level="ghost">
									<XIcon size={18} />
								</Button>
							</DialogPrimitive.Close>
						</div>
						<DialogPrimitive.Description>
							Select an installed Java runtime to use.
						</DialogPrimitive.Description>
						{jvms.isError ? (
							<p className="text-sm text-text-secondary">
								Unable to load installed Java runtimes.
							</p>
						) : (
							<Dropdown
								className="w-full"
								disabled={jvms.isLoading || !jvms.data?.length}
								aria-busy={jvms.isLoading}
								value={detectedPath}
								onChange={(event) => {
									setDetectedPath(event.target.value);
									onChange(event.target.value);
									onBlur();
									setSelectorOpen(false);
								}}
							>
								<option value="" disabled>
									{jvms.isLoading
										? "Loading Java runtimes..."
										: !jvms.data?.length
											? "No Java runtimes found"
											: "Select a Java runtime"}
								</option>
								{jvms.data?.map((jvm) => (
									<option key={jvm.path} value={jvm.path}>
										Java {jvm.major_version} -{" "}
										{jvm.version_string} - {jvm.vendor} (
										{jvm.arch})
									</option>
								))}
							</Dropdown>
						)}
					</DialogContent>
				</DialogPortal>
			</DialogRoot>
		</>
	);
}
