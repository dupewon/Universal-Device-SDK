#include "main.h"
#include "uds.h"

UDS_HandleTypeDef huds;
UART_HandleTypeDef huart2;

static void UDS_LedControl(UDS_Message* msg) {
    int led_state = msg->data[0];
    HAL_GPIO_WritePin(LED_GPIO_Port, LED_Pin, led_state ? GPIO_PIN_SET : GPIO_PIN_RESET);
}

int main(void) {
    HAL_Init();
    SystemClock_Config();
    MX_USART2_UART_Init();

    UDS_Init(&huds, &huart2);
    UDS_RegisterMethod(&huds, "SetLed", UDS_LedControl);
    UDS_Start(&huds);

    while (1) {
        UDS_Process(&huds, 100);
    }
}
