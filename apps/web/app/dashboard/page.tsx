import Link from "next/link";

export default function Home() {
	return (
		<div>
			<Link href="/servers/1"> Server 1</Link>
			<Link href="/servers/2"> Server 2</Link>
			<Link href="/servers/3"> Server 3</Link>
		</div>
	);
}
