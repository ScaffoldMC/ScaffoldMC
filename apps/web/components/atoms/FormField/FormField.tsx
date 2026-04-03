export default function FormField({ children }: { children: React.ReactNode }) {
	return (
		<div className="flex w-full flex-col items-start justify-start gap-1">
			{children}
		</div>
	);
}
