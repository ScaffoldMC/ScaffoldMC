import { ReactNode } from "react";

export function InfoNugget({
	title,
	value,
	children,
}: {
	title: string;
	value: string;
	children?: ReactNode;
}) {
	return (
		<div className="flex flex-row gap-4 p-2 bg-surface rounded-md border border-border-static">
			<div className="flex flex-col gap-2">
				<span className="mr-2">{title}</span>
				<span className="text-xl">{value}</span>
			</div>
			{children}
		</div>
	);
}
