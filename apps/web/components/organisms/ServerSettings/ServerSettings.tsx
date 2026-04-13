import FormField from "@/components/atoms/FormField/FormField";
import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import { VersionSelector } from "../VersionSelector/VersionSelector";
import { Button } from "@/components/atoms/Button/Button";
import { SubmitEventHandler, useState } from "react";
import { ListInput } from "@/components/atoms/ListInput/ListInput";

export function ServerSettings({ serverId }: { serverId: string }) {
	// TODO: Diff original values with current values to determine if form is modified
	const [formModified, setFormModified] = useState(false);

	const [newStopCommand, setNewStopCommand] = useState("shutdown");
	const [args, setArgs] = useState<string[]>(["--no-gui"]);

	const submitHandler: SubmitEventHandler<HTMLFormElement> = (event) => {
		event.preventDefault();
		console.log("Updated");
	};

	return (
		<div className="flex flex-col gap-2">
			<h2>Configuration</h2>

			<form
				className="flex flex-col gap-4 p-4 bg-surface rounded-md border border-border-static"
				onSubmit={submitHandler}
			>
				<FormField>
					<Label htmlFor="name">Name</Label>
					<TextInput name="name" />
				</FormField>
				<FormField>
					<Label htmlFor="stopcommand">Stop Command</Label>
					<TextInput
						name="stopcommand"
						value={newStopCommand}
						onChange={(e) => setNewStopCommand(e.target.value)}
					/>
				</FormField>
				<FormField>
					<Label htmlFor="args">Args</Label>
					<ListInput value={args} onChange={setArgs} />
				</FormField>
				<FormField>
					<Label>Game</Label>
					<VersionSelector onGame={() => {}} />
				</FormField>

				<div className="self-end flex flex-row gap-2">
					<Button hidden={!formModified} level="secondary">
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
