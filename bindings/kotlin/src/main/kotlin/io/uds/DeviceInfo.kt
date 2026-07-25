package io.uds

import com.google.gson.annotations.SerializedName

data class DeviceInfo(
    val id: String,
    val name: String? = null,
    val transport: String? = null,
    val connected: Boolean = false,
    @SerializedName("firmware_version")
    val firmwareVersion: String? = null,
    val platform: String? = null,
    val capabilities: Map<String, Any>? = null,
    @SerializedName("uptime_seconds")
    val uptimeSeconds: Long? = null
)
