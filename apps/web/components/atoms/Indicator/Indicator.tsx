import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

export interface IndicatorProps {
	state: IndicatorState;
}

export type IndicatorState = "success" | "working" | "error";

const indicatorStyles = cva(cn("m-1 size-1.5 rounded-full"), {
	variants: {
		state: {
			success: "border border-border-static bg-[#00ff00]",
			working: "border border-border-static bg-[#ffcc00]",
			error: "border border-border-static bg-[#ff0000]",
		},
	},
});

export function Indicator(props: IndicatorProps) {
	const className = indicatorStyles({ state: props.state });
	return <div className={className}></div>;
}
