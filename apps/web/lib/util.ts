import clsx from "clsx";
import { ClassNameValue, twMerge } from "tailwind-merge";
import { Game } from "./servertypes";

export function singularOrPlural(
	count: number,
	singular: string,
	plural: string,
): string {
	return count === 1 ? singular : plural;
}

export function makeInitials(name: string): string {
	return name
		.split(" ")
		.map((part) => part.charAt(0).toUpperCase())
		.join("");
}

export function cn(...inputs: ClassNameValue[]): string {
	return twMerge(clsx(inputs));
}

export function gameString(game: Game): string {
	switch (game.type) {
		case "minecraft_java": {
			let loaderString = "";

			if (game.loader !== "vanilla") {
				if ("fabric" in game.loader) {
					loaderString = `with Fabric ${game.loader.fabric.loader} (${game.loader.fabric.launcher})`;
				} else if ("paper" in game.loader) {
					loaderString = `with Paper ${game.loader.paper.build}`;
				}
			}

			return `Minecraft Java ${game.version} ${loaderString}`;
		}
		default:
			return "Unknown Game";
	}
}
