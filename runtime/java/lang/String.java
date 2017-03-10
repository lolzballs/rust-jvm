package java.lang;

public final class String {
    private final char[] bytes;

    public String(char[] bytes) {
        this.bytes = new char[bytes.length];
        System.arraycopy(bytes, 0, this.bytes, 0, bytes.length);
    }

    public byte[] getBytes() {
        byte[] b = new byte[2 * bytes.length];
        for (int i = 0; i < bytes.length; i++) {
            b[2 * i] = (byte) ((bytes[i] & 0xff00) >>> 8);
            b[2 * i + 1] = (byte) (bytes[i] & 0x00ff);
        }
        return b;
    }
}
