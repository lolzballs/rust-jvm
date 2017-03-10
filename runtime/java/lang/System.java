package java.lang;

public class System {
    public static native void arraycopy(Object src, int srcPos, Object dest, int destPos, int length);

    public static native int readInt();
    public static native void write(byte b);

    public static void println(String s) {
        byte[] b;
        b = s.getBytes();
        for (byte c : b) {
            write(c);
        }

        write((byte) '\n');
    }

    public static void println(int i) {
        System.println("FUCK");
    }
}
