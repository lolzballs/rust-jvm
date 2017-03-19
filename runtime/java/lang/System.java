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

    public static void println(int n) {
        int length = (int) (Math.log10(n) + 1);
        for (int i = length - 1; i >= 0; i--) {
            int dig = (int) (((float) n / Math.pow(10, i)) % 10);
            write((byte) ((byte) '0' + dig));
        }
        write((byte) '\n');
    }
}
