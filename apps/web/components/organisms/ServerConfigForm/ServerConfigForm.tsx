import FormField from "@/components/atoms/FormField/FormField";
import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Button } from "@/components/atoms/Button/Button";
import { ListInput } from "@/components/atoms/ListInput/ListInput";
import { Controller, useForm } from "react-hook-form";
import { useServer } from "@/hooks/servers";
import { VersionChanger } from "../VersionChanger/VersionChanger";
import { useEffect } from "react";

export function ServerConfigForm({ serverId }: { serverId: string }) {
	const { server, mutateConfig } = useServer(serverId);

	const {
		register,
		handleSubmit,
		control,
		reset,
		formState: { touchedFields },
	} = useForm({
		mode: "onBlur",
		defaultValues: {
			name: server.data?.config.name,
			stop_command: server.data?.config.stop_command,
			args: server.data?.config.args,
			game: server.data?.config.game,
		},
	});

	const onSubmit = handleSubmit((data) => {
		mutateConfig(data);
	});

	const formModified = Object.keys(touchedFields).length > 0;

	// Used to reset form values when server config changes
	useEffect(() => {
		reset({
			name: server.data.config.name,
			stop_command: server.data.config.stop_command,
			args: server.data.config.args,
			game: server.data.config.game,
		});
	}, [server.data?.config, reset]);

	return (
		<div className="flex flex-col gap-4 p-4 bg-surface rounded-md border border-border-static">
			<FormField>
				<Label htmlFor="name">Name</Label>
				<TextInput name="name" {...register("name")} />
			</FormField>
			<FormField>
				<Label htmlFor="stop_command">Stop Command</Label>
				<TextInput name="stop_command" {...register("stop_command")} />
			</FormField>
			<FormField>
				<Label htmlFor="args">Args</Label>
				<Controller
					name="args"
					control={control}
					render={({ field }) => (
						<ListInput
							value={field.value}
							onChange={(value) => {
								field.onBlur();
								field.onChange(value);
							}}
						/>
					)}
				/>
			</FormField>
			<FormField>
				<Label htmlFor="game">Game</Label>
				<Controller
					name="game"
					control={control}
					render={({ field }) => (
						<VersionChanger
							value={field.value}
							onChange={(value) => {
								field.onBlur();
								field.onChange(value);
							}}
						/>
					)}
				/>
			</FormField>

			<div className="self-end flex flex-row gap-2">
				<Button
					hidden={!formModified}
					level="secondary"
					onClick={() => reset()}
				>
					Revert
				</Button>
				<Button
					disabled={!formModified}
					level="primary"
					onClick={onSubmit}
				>
					Save
				</Button>
			</div>
		</div>
	);
}
