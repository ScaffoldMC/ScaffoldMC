import { Dropdown } from "@/components/atoms/Dropdown/Dropdown";
import styles from "./VersionSelector.module.css";
import { useQuery } from "@tanstack/react-query";
import api from "@/lib/axios";
import { useState, useMemo } from "react";
import { CompleteVersionResponse, OptionsResponse } from "@/lib/servertypes";

type VersionResponse = CompleteVersionResponse | OptionsResponse;

export function VersionSelector() {
	const [path, setPath] = useState<string[]>([]);
	const [game, setGame] = useState<CompleteVersionResponse | null>(null);
	const [levelCache, setLevelCache] = useState<
		Record<number, OptionsResponse>
	>({});

	const currentRoute =
		path.length > 0 ? `/game-versions/${path.join("/")}` : "/game-versions";

	const currentLevel = useQuery<VersionResponse>({
		queryKey: ["versions", currentRoute],
		queryFn: async () => {
			const res = await api.get(currentRoute);
			return res.data;
		},
		retry: false,
	});

	// Reached complete version response
	if (currentLevel.data && "game" in currentLevel.data && !game) {
		setGame(currentLevel.data);
	}

	// Cache options for each selection level
	if (
		currentLevel.data &&
		"options" in currentLevel.data &&
		"message" in currentLevel.data &&
		!levelCache[path.length]
	) {
		// Appeasing typescript
		const options = currentLevel.data as OptionsResponse;

		setLevelCache((prev) => ({
			...prev,
			[path.length]: options,
		}));
	}

	// Memoize levels to display
	const levels = useMemo(() => {
		const displayLevels: number[] = [];

		// Add dropdown for each current selection
		for (let i = 0; i < path.length; i++) {
			displayLevels.push(i);
		}

		// Add one more dropdown if current level isn't the end
		if (currentLevel.data && "options" in currentLevel.data) {
			displayLevels.push(path.length);
		}

		return displayLevels;
	}, [path.length, currentLevel.data]);

	const handleSelectChange = (levelIndex: number, value: string) => {
		setGame(null);

		if (!value) {
			const newPath = path.slice(0, levelIndex);
			setPath(newPath);
			return;
		}

		const newPath = [...path.slice(0, levelIndex), value];
		setPath(newPath);

		// Clear cache for levels beyond the changed level
		setLevelCache((prev) => {
			const newCache = { ...prev };
			Object.keys(newCache).forEach((key) => {
				if (Number(key) > levelIndex) {
					delete newCache[Number(key)];
				}
			});
			return newCache;
		});
	};

	const getLevelOptions = (levelIndex: number): string[] => {
		return levelCache[levelIndex]?.options || [];
	};

	const getLevelMessage = (levelIndex: number): string => {
		return levelCache[levelIndex]?.message || "";
	};

	return (
		<div className={styles.container}>
			{levels.map((levelIndex) => (
				<Dropdown
					key={levelIndex}
					onChange={(e) =>
						handleSelectChange(levelIndex, e.target.value)
					}
					value={path[levelIndex] || ""}
				>
					<option value="">{getLevelMessage(levelIndex)}</option>
					{getLevelOptions(levelIndex)?.map((option: string) => (
						<option key={option} value={option}>
							{option}
						</option>
					))}
				</Dropdown>
			))}
		</div>
	);
}
