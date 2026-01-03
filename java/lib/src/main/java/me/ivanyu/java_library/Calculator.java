package me.ivanyu.java_library;

public class Calculator {
    public static void add(int a, int b, ResultCallback callback) {
        int sum = a + b;
        String message = "Result: " + sum;
        Result result = new Result(message);
        callback.onResult(result);
    }
}
