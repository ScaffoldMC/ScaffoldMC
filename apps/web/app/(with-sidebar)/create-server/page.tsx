"use client";

import { Button } from "@/components/atoms/Button/Button";
import { useRouter } from "next/navigation";
import styles from "./page.module.css";
import { Import, Plus } from "lucide-react";

export default function CreateServerPage() {
	const router = useRouter();

	const handleClick = () => {
		router.push("/create-server/new");
	};

	return (
		<div className={styles.layout}>
			<h1>Select creation option</h1>
			<div className={styles.createOptions}>
				<Button onClick={handleClick} level="secondary" size="variable">
					<div className={styles.createOption}>
						<Plus size={32} />
						<p>Create new server</p>
					</div>
				</Button>
				<Button
					onClick={handleClick}
					level="secondary"
					size="variable"
					disabled
				>
					<div className={styles.createOption}>
						<Import size={32} />
						<p>Import existing server</p>
					</div>
				</Button>
			</div>
		</div>
	);
}
