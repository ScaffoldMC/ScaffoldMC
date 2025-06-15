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
