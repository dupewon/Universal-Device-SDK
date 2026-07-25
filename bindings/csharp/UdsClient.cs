using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Threading.Tasks;

namespace UniversalDeviceSdk
{
    public class UdsClient : IDisposable
    {
        private IntPtr ctx;
        private bool disposed = false;

        public UdsClient()
        {
            ctx = uds_init();
        }

        public UdsClient(string configPath)
        {
            ctx = uds_init_with_config(configPath);
        }

        public List<UdsDevice> ListDevices()
        {
            string json = Marshal.PtrToStringAnsi(uds_list_devices(ctx));
            return ParseDevices(json);
        }

        public UdsDevice InspectDevice(string deviceId)
        {
            string json = Marshal.PtrToStringAnsi(uds_inspect_device(ctx, deviceId));
            return ParseDevice(json);
        }

        public async Task<bool> FlashAsync(string deviceId, string firmwarePath)
        {
            return await Task.Run(() => uds_flash(ctx, deviceId, firmwarePath) == 0);
        }

        public string Rpc(string deviceId, string method, string jsonParams)
        {
            return Marshal.PtrToStringAnsi(uds_rpc(ctx, deviceId, method, jsonParams));
        }

        public void Dispose()
        {
            if (!disposed && ctx != IntPtr.Zero)
            {
                uds_destroy(ctx);
                ctx = IntPtr.Zero;
                disposed = true;
            }
        }

        private List<UdsDevice> ParseDevices(string json)
        {
            var devices = new List<UdsDevice>();
            if (string.IsNullOrEmpty(json)) return devices;
            return System.Text.Json.JsonSerializer.Deserialize<List<UdsDevice>>(json) ?? devices;
        }

        private UdsDevice ParseDevice(string json)
        {
            if (string.IsNullOrEmpty(json)) return null;
            return System.Text.Json.JsonSerializer.Deserialize<UdsDevice>(json);
        }

        [DllImport("uds_c")]
        private static extern IntPtr uds_init();

        [DllImport("uds_c")]
        private static extern IntPtr uds_init_with_config(string configPath);

        [DllImport("uds_c")]
        private static extern void uds_destroy(IntPtr ctx);

        [DllImport("uds_c")]
        private static extern IntPtr uds_list_devices(IntPtr ctx);

        [DllImport("uds_c")]
        private static extern IntPtr uds_inspect_device(IntPtr ctx, string deviceId);

        [DllImport("uds_c")]
        private static extern int uds_flash(IntPtr ctx, string deviceId, string firmwarePath);

        [DllImport("uds_c")]
        private static extern IntPtr uds_rpc(IntPtr ctx, string deviceId, string method, string jsonParams);
    }
}
