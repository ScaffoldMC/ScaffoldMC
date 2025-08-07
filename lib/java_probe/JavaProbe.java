public class JavaProbe {
	public static void main(String[] args) {
		String version = System.getProperty("java.version");
		String arch = System.getProperty("os.arch");
		String vendor = System.getProperty("java.vendor");

		String majorVersion = version.split("\\.")[0];
		if (majorVersion.equals("1")) {
			majorVersion = version.split("\\.")[1];
		}

		System.out.println("{");
		System.out.println("	\"javaMajorVersion\": \"" + majorVersion + "\",");
		System.out.println("	\"architecture\": \"" + arch + "\",");
		System.out.println("	\"vendor\": \"" + vendor + "\"");
		System.out.println("}");
	}

}
