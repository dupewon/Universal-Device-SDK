package io.uds;

import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;

public class UdsClient implements AutoCloseable {
    private long ctx;
    private final ExecutorService executor = Executors.newSingleThreadExecutor();
    private final List<Consumer<String>> logListeners = new CopyOnWriteArrayList<>();

    public UdsClient() {
        this.ctx = udsInit();
    }

    public UdsClient(String configPath) {
        this.ctx = udsInitWithConfig(configPath);
    }

    public List<DeviceInfo> listDevices() {
        String json = udsListDevices(ctx);
        return DeviceInfo.fromJsonArray(json);
    }

    public DeviceInfo inspectDevice(String deviceId) {
        String json = udsInspectDevice(ctx, deviceId);
        return DeviceInfo.fromJson(json);
    }

    public CompletableFuture<Boolean> flash(String deviceId, String firmwarePath) {
        return CompletableFuture.supplyAsync(() ->
            udsFlash(ctx, deviceId, firmwarePath) == 0, executor);
    }

    public String rpc(String deviceId, String method, String params) {
        return udsRpc(ctx, deviceId, method, params);
    }

    public void subscribeLogs(Consumer<String> listener) {
        logListeners.add(listener);
    }

    public void close() {
        if (ctx != 0) {
            udsDestroy(ctx);
            ctx = 0;
        }
        executor.shutdown();
    }

    private native long udsInit();
    private native long udsInitWithConfig(String configPath);
    private native void udsDestroy(long ctx);
    private native String udsListDevices(long ctx);
    private native String udsInspectDevice(long ctx, String deviceId);
    private native int udsFlash(long ctx, String deviceId, String firmwarePath);
    private native String udsRpc(long ctx, String deviceId, String method, String params);
}
