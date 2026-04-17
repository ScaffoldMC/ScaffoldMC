import FormField from "@/components/atoms/FormField/FormField";
import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { Button } from "@/components/atoms/Button/Button";
import { ListInput } from "@/components/atoms/ListInput/ListInput";
import { Controller, useForm } from "react-hook-form";
import { useServer } from "@/hooks/servers";
import { VersionChanger } from "../VersionChanger/VersionChanger";

export function ServerSettings({ serverId }: { serverId: string }) {
	const { server } = useServer(serverId);

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
			stopCommand: server.data?.config.stop_command,
			args: server.data?.config.args,
			game: server.data?.config.game,
		},
	});

	// TODO: Custom fields need proper change detection
	// TODO: Game field needs to be rethought to allow for default value. Perhaps a wrapper component?

	const onSubmit = handleSubmit((data) => {
		console.log("Updated");
	});

	const formModified = Object.keys(touchedFields).length > 0;

	return (
		<div className="flex flex-col gap-2">
			<h2>Configuration</h2>

			<form
				className="flex flex-col gap-4 p-4 bg-surface rounded-md border border-border-static"
				onSubmit={onSubmit}
			>
				<FormField>
					<Label htmlFor="name">Name</Label>
					<TextInput name="name" {...register("name")} />
				</FormField>
				<FormField>
					<Label htmlFor="stopCommand">Stop Command</Label>
					<TextInput
						name="stopCommand"
						{...register("stopCommand")}
					/>
				</FormField>
				<FormField>
					<Label htmlFor="args">Args</Label>
					<Controller
						name="args"
						control={control}
						render={({ field }) => (
							<ListInput
								value={field.value}
								onChange={field.onChange}
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
								onChange={field.onChange}
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
						type="submit"
						level="primary"
					>
						Save
					</Button>
				</div>
			</form>
		</div>
	);
}
