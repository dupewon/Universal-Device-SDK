#include <stdio.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "driver/uart.h"
#include "uds.h"

static uds_context_t* uds_ctx;

void uds_server_task(void* pvParameters) {
    uds_ctx = uds_init();
    uds_transport_t* transport = uds_transport_create(UART_NUM_0, 115200);
    uds_server_start(uds_ctx, transport);

    while (1) {
        uds_server_poll(uds_ctx, 100);
    }
}

void app_main(void) {
    xTaskCreate(uds_server_task, "uds_server", 4096, NULL, 5, NULL);
}
