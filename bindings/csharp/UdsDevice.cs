using System.Collections.Generic;

namespace UniversalDeviceSdk
{
    public class UdsDevice
    {
        public string Id { get; set; }
        public string Name { get; set; }
        public string Platform { get; set; }
        public string TransportType { get; set; }
        public string Address { get; set; }
        public bool Connected { get; set; }
        public string FirmwareVersion { get; set; }
        public Dictionary<string, bool> Capabilities { get; set; }

        public UdsDevice()
        {
            Capabilities = new Dictionary<string, bool>();
        }

        public override string ToString()
        {
            return $"UdsDevice[Id={Id}, Name={Name}, Platform={Platform}, Connected={Connected}]";
        }
    }
}
