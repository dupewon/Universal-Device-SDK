package io.uds

import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import kotlinx.coroutines.*
import java.util.concurrent.Executors

class UdsClient(private var ctx: Long = 0) : AutoCloseable {
    private val executor = Executors.newSingleThreadExecutor()
    private val dispatcher = executor.asCoroutineDispatcher()

    constructor(configPath: String? = null) : this(if (configPath != null) udsInitWithConfig(configPath) else udsInit())

    fun listDevices(): List<DeviceInfo> {
        val json = udsListDevices(ctx)
        return Gson().fromJson(json, object : TypeToken<List<DeviceInfo>>() {}.type)
    }

    fun inspectDevice(deviceId: String): DeviceInfo {
        val json = udsInspectDevice(ctx, deviceId)
        return Gson().fromJson(json, DeviceInfo::class.java)
    }

    suspend fun flash(deviceId: String, firmwarePath: String): Boolean =
        withContext(dispatcher) { udsFlash(ctx, deviceId, firmwarePath) == 0 }

    fun rpc(deviceId: String, method: String, params: String): String =
        udsRpc(ctx, deviceId, method, params)

    override fun close() {
        if (ctx != 0L) {
            udsDestroy(ctx)
            ctx = 0L
        }
        executor.shutdown()
    }

    private external fun udsInit(): Long
    private external fun udsInitWithConfig(configPath: String): Long
    private external fun udsDestroy(ctx: Long)
    private external fun udsListDevices(ctx: Long): String
    private external fun udsInspectDevice(ctx: Long, deviceId: String): String
    private external fun udsFlash(ctx: Long, deviceId: String, firmwarePath: String): Int
    private external fun udsRpc(ctx: Long, deviceId: String, method: String, params: String): String
}
