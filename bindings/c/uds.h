#ifndef UDS_H
#define UDS_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#define UDS_MAGIC { 0x55, 0x44, 0x53, 0x21 }
#define UDS_PROTOCOL_VERSION_MAJOR 1
#define UDS_PROTOCOL_VERSION_MINOR 0
#define UDS_MAX_PAYLOAD_SIZE 65535

typedef struct uds_context uds_context_t;
typedef struct uds_device uds_device_t;
typedef struct uds_transport uds_transport_t;

uds_context_t* uds_init(void);
void uds_destroy(uds_context_t* ctx);

int uds_discover(uds_context_t* ctx, uds_device_t** devices, size_t* count, uint32_t timeout_ms);
int uds_connect(uds_context_t* ctx, uds_device_t* device);
int uds_disconnect(uds_context_t* ctx, uds_device_t* device);

int uds_flash(uds_context_t* ctx, uds_device_t* device, const uint8_t* firmware, size_t size);
int uds_monitor_start(uds_context_t* ctx, uds_device_t* device, void (*callback)(const char* line));
int uds_monitor_stop(uds_context_t* ctx, uds_device_t* device);

int uds_rpc_call(uds_context_t* ctx, uds_device_t* device,
                 const char* method, const uint8_t* params, size_t params_len,
                 uint8_t** result, size_t* result_len);

void uds_free_string(char* s);
void uds_free_devices(uds_device_t* devices, size_t count);

#endif /* UDS_H */
