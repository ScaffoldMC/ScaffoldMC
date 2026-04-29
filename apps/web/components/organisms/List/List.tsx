"use client";

import { useEffect, useRef, useState } from "react";
import { cn, singularOrPlural } from "@/lib/util";

interface ListProps {
	hideHeader?: boolean;
	children: React.ReactNode;
	emptyView?: React.ReactNode;
	names?: {
		singular: string;
		plural: string;
	};
}

export function List({ hideHeader, children, emptyView, names }: ListProps) {
	const contentRef = useRef<HTMLDivElement>(null);
	const [numItems, setNumItems] = useState<number>(0);

	// Update numItems when items are added/removed
	useEffect(() => {
		if (contentRef.current) {
			// Count immediate children that are ListItems
			const countItems = contentRef.current.children.length;
			setNumItems(countItems);
		}
	}, [children]);

	return (
		<div className="overflow-hidden rounded-md border border-border-static bg-surface">
			{!hideHeader && (
				<div className="flex flex-row border-b border-border-static p-2 text-text-primary">
					<b>
						{numItems}{" "}
						{singularOrPlural(
							numItems,
							names?.singular || "Item",
							names?.plural || "Items",
						)}
					</b>
				</div>
			)}
			<div
				className="flex w-full flex-col [&>*:last-child]:border-b-0"
				ref={contentRef}
			>
				{children}
			</div>

			{numItems === 0 && emptyView}
		</div>
	);
}

export function ListItem({
	children,
	className,
}: {
	children: React.ReactNode;
	className?: string;
}) {
	return (
		<div
			className={cn(
				"border-b border-border-static bg-surface",
				className,
			)}
		>
			{children}
		</div>
	);
}

List.Item = ListItem;
