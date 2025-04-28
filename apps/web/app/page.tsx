import { redirect } from "next/navigation";

async function isUserLoggedIn(): Promise<boolean> {
	// TODO: Actual authentication logic
	return false;
}

export default async function Home() {
	const loggedIn = await isUserLoggedIn();

	if (loggedIn) {
		redirect("/dashboard");
	} else {
		redirect("/login");
	}
}
