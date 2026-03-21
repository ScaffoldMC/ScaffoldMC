import { cva } from "class-variance-authority";
import { cn } from "@/lib/util";

export interface IndicatorProps {
	state: IndicatorState;
}

export type IndicatorState = "success" | "working" | "error";

const indicatorStyles = cva(cn("m-1 size-1.5 rounded-full"), {
	variants: {
		state: {
			success: "bg-[#00ff00] shadow-[0_0_2px_1px_rgba(0,255,0,0.5)]",
			working: "bg-[#ffcc00] shadow-[0_0_2px_1px_rgba(255,204,0,0.5)]",
			error: "bg-[#ff0000] shadow-[0_0_2px_1px_rgba(255,0,0,0.5)]",
		},
	},
});

export function Indicator(props: IndicatorProps) {
	const className = indicatorStyles({ state: props.state });
	return <div className={className}></div>;
}
