#include "uds.h"
#include <stdlib.h>
#include <string.h>

struct uds_context {

};

struct uds_device {

};

uds_context_t* uds_init(void) {

    uds_context_t* ctx = calloc(1, sizeof(uds_context_t));
    return ctx;
}

void uds_destroy(uds_context_t* ctx) {
    free(ctx);
}

int uds_discover(uds_context_t* ctx, uds_device_t** devices, size_t* count, uint32_t timeout_ms) {
    (void)ctx;
    (void)devices;
    (void)count;
    (void)timeout_ms;
    return -1;
}

int uds_connect(uds_context_t* ctx, uds_device_t* device) {
    (void)ctx;
    (void)device;
    return -1;
}

int uds_disconnect(uds_context_t* ctx, uds_device_t* device) {
    (void)ctx;
    (void)device;
    return -1;
}

int uds_flash(uds_context_t* ctx, uds_device_t* device, const uint8_t* firmware, size_t size) {
    (void)ctx;
    (void)device;
    (void)firmware;
    (void)size;
    return -1;
}

int uds_monitor_start(uds_context_t* ctx, uds_device_t* device, void (*callback)(const char*)) {
    (void)ctx;
    (void)device;
    (void)callback;
    return -1;
}

int uds_monitor_stop(uds_context_t* ctx, uds_device_t* device) {
    (void)ctx;
    (void)device;
    return -1;
}

int uds_rpc_call(uds_context_t* ctx, uds_device_t* device,
                 const char* method, const uint8_t* params, size_t params_len,
                 uint8_t** result, size_t* result_len) {
    (void)ctx;
    (void)device;
    (void)method;
    (void)params;
    (void)params_len;
    (void)result;
    (void)result_len;
    return -1;
}

void uds_free_string(char* s) {
    free(s);
}

void uds_free_devices(uds_device_t* devices, size_t count) {
    (void)devices;
    (void)count;
}
