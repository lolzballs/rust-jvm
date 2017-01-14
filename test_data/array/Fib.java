import java.util.Arrays;

public class Fib {
    public static int[] dp = new int[100];

    static {
        dp[1] = 1;
        dp[2] = 1;
    }

    public static int fib(int n) {
        if (dp[n] != 0) return dp[n];
        return dp[n] = fib(n - 1) + fib(n - 2);
    }   
}
