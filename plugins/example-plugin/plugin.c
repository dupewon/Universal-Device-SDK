#include <stdint.h>

#define UDS_PLUGIN_ABI_VERSION 1

uint32_t uds_plugin_abi_version(void) {
    return UDS_PLUGIN_ABI_VERSION;
}

const char* uds_plugin_name(void) {
    return "example-plugin-c";
}

const char* uds_plugin_version(void) {
    return "0.1.0";
}

int32_t uds_plugin_register(void* registry) {
    (void)registry;
    return 0;
}

int32_t uds_plugin_unregister(void) {
    return 0;
}
