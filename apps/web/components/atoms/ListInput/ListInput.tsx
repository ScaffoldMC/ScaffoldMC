import { cn } from "@/lib/util";
import { TextInput } from "../TextInput/TextInput";
import { Button } from "../Button/Button";
import { Plus, XIcon } from "lucide-react";

export interface ListInputProps
	extends Omit<React.ComponentPropsWithoutRef<"div">, "onChange"> {
	value: string[];
	onChange: (newValue: string[]) => void;
}

export function ListInput({ value, onChange, className }: ListInputProps) {
	return (
		<div className={cn("flex flex-row gap-1", className)}>
			{value.map((v, i) => (
				<div className="relative" key={i}>
					<TextInput
						value={v}
						onChange={(e) => {
							const newValue = [...value];
							newValue[i] = e.target.value;
							onChange(newValue);
						}}
					/>
					<Button
						className="absolute right-0 top-0 rounded-l-none"
						level="ghost"
						onClick={() =>
							onChange([
								...value.slice(0, i),
								...value.slice(i + 1),
							])
						}
					>
						<XIcon size={16} />
					</Button>
				</div>
			))}
			<Button level="secondary" onClick={() => onChange([...value, ""])}>
				<Plus size={18} />
			</Button>
		</div>
	);
}
