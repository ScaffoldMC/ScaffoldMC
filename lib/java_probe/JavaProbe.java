public class JavaProbe {
	public static void main(String[] args) {
		String version = System.getProperty("java.version");
		String arch = System.getProperty("os.arch");
		String vendor = System.getProperty("java.vendor");

		String majorVersion = version.split("\\.")[0];
		if (majorVersion.equals("1")) {
			majorVersion = version.split("\\.")[1];
		}

		System.out.print("{");
		System.out.print("\"version_string\": \"" + version + "\",");
		System.out.print("\"major_version\": " + majorVersion + ",");
		System.out.print("\"arch\": \"" + arch + "\",");
		System.out.print("\"vendor\": \"" + vendor + "\"");
		System.out.print("}");
	}

}
