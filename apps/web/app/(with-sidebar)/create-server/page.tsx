"use client";

import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import styles from "./page.module.css";
import { Button } from "@/components/atoms/Button/Button";
import { FormEvent, useState } from "react";
import { Game } from "@/lib/servertypes";
import api from "@/lib/axios";
import { VersionSelector } from "@/components/organisms/VersionSelector/VersionSelector";
import { useServers } from "@/hooks/servers";

export default function CreateServerPage() {
	const [game, setGame] = useState<Game | null>(null);
	const { mutateServers } = useServers();

	const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		const formData = new FormData(event.currentTarget);
		const name = formData.get("name") as string;

		if (!game || !name) return; // TODO: Show error

		// TODO: Show loading state
		mutateServers({ name, game }).then((res) => {
			// temp
			if (res.status === 201) {
				alert("Server created successfully!");
			}
		});
	};

	const handleGame = (selectedGame?: Game) => {
		setGame(selectedGame);
	};

	return (
		<div className={styles.layout}>
			<form className={styles.form} onSubmit={handleSubmit}>
				<div className={styles.field}>
					<Label>Server Software</Label>
					<VersionSelector onGame={handleGame} />
				</div>
				<div className={styles.field}>
					<Label>Server Name</Label>
					<TextInput name="name" />
				</div>
				<Button type="submit" level="primary">
					Create
				</Button>
			</form>
		</div>
	);
}
