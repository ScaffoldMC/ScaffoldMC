"use client";

import { Label } from "@/components/atoms/Label/Label";
import { TextInput } from "@/components/atoms/TextInput/TextInput";
import styles from "./page.module.css";
import { Button } from "@/components/atoms/Button/Button";
import { FormEvent, useState } from "react";
import { Game } from "@/lib/servertypes";
import api from "@/lib/axios";
import { VersionSelector } from "@/components/organisms/VersionSelector/VersionSelector";

export default function CreateServerPage() {
	const [game, setGame] = useState<Game>(null);

	const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		console.log(game);
	};

	const handleGame = (selectedGame: Game) => {
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
					<TextInput />
				</div>
				<Button type="submit" level="primary">
					Create
				</Button>
			</form>
		</div>
	);
}
