import {
	Avatar,
	AvatarFallback,
	AvatarImage,
} from "@/components/atoms/avatar/avatar";

import styles from "./page.module.css";

export default async function Page({
	params,
}: {
	params: Promise<{ slug: string }>;
}) {
	const { slug } = await params;
	return (
		<div>
			<div className={styles.title}>
				<Avatar size={64} shape="square">
					<AvatarFallback>?</AvatarFallback>
					<AvatarImage src="/images/server-default.png" />
				</Avatar>
				<div className={styles.titleInfo}>
					<h1>Server Name</h1>
					<p>Additional info</p>
				</div>
			</div>
		</div>
	);
}
