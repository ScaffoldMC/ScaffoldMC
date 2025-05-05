"use client";

import { useEffect, useRef, useState } from "react";
import styles from "./list.module.css";
import { singularOrPlural } from "@/lib/util";

interface ListProps {
	children: React.ReactNode;
	names: {
		singular: string;
		plural: string;
	};
}

export function List({ children, names }: ListProps) {
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
		<div className={styles.root}>
			<div className={styles.header}>
				<b>
					{numItems}{" "}
					{singularOrPlural(numItems, names.singular, names.plural)}
				</b>
			</div>
			<div className={styles.content} ref={contentRef}>
				{children}
			</div>
		</div>
	);
}

export function ListItem({ children }: { children: React.ReactNode }) {
	return <div className={styles.item}>{children}</div>;
}

List.Item = ListItem;
