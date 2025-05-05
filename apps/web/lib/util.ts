export function singularOrPlural(
	count: number,
	singular: string,
	plural: string,
): string {
	return count === 1 ? singular : plural;
}
