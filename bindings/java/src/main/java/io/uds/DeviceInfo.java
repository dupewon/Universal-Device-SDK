package io.uds;

import java.util.*;
import com.google.gson.*;
import com.google.gson.reflect.*;

public class DeviceInfo {
    public String id;
    public String name;
    public String transport;
    public Boolean connected;
    public String firmwareVersion;
    public String platform;
    public Map<String, Object> capabilities;
    public Long uptimeSeconds;

    public static List<DeviceInfo> fromJsonArray(String json) {
        Gson gson = new Gson();
        Type listType = new TypeToken<List<DeviceInfo>>(){}.getType();
        return gson.fromJson(json, listType);
    }

    public static DeviceInfo fromJson(String json) {
        return new Gson().fromJson(json, DeviceInfo.class);
    }

    public String toJson() {
        return new Gson().toJson(this);
    }

    @Override
    public String toString() {
        return String.format("DeviceInfo{id='%s', name='%s', connected=%s, platform='%s'}",
            id, name, connected, platform);
    }
}
