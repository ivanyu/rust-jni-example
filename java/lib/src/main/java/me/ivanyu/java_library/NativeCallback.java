package me.ivanyu.java_library;

public class NativeCallback implements ResultCallback {
    @Override
    // Mind the `native` keyword.
    public native void onResult(Result result);
}
