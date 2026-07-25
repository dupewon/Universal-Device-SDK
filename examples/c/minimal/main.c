#include "uds.h"
#include <stdio.h>

int main(void) {
    uds_context_t* ctx = uds_init();
    if (!ctx) {
        fprintf(stderr, "Failed to initialize UDS\n");
        return 1;
    }

    uds_device_t* devices = NULL;
    size_t count = 0;
    int ret = uds_discover(ctx, &devices, &count, 5000);
    if (ret != 0) {
        fprintf(stderr, "Discovery failed: %d\n", ret);
    } else {
        printf("Found %zu device(s)\n", count);
    }

    uds_free_devices(devices, count);
    uds_destroy(ctx);
    return 0;
}
