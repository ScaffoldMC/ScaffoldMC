import Link from "next/link";

export default function Dashboard() {
	return (
		<div>
			<div>
				<Link href="/dashboard/1"> Server 1</Link>
				<Link href="/dashboard/2"> Server 2</Link>
				<Link href="/dashboard/3"> Server 3</Link>
			</div>
		</div>
	);
}
