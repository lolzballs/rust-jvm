public class Instance {
    public int value;

    public Instance(int value) {
        this.value = value;
    }

    public static int setAndGetValue(int a) {
        Instance instance = new Instance(420);
        instance.value = a;

        return instance.value;
    }
}
