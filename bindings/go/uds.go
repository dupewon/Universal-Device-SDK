package uds

/*
#cgo LDFLAGS: -luds_c
#include "uds.h"
*/
import "C"
import (
	"fmt"
	"time"
	"unsafe"
)

type DeviceInfo struct {
	ID              string
	Name            string
	Platform        string
	Transport       string
	Connected       bool
	FirmwareVersion string
	UptimeSeconds   int64
}

type Client struct {
	ctx *C.uds_context_t
}

func NewClient() *Client {
	return &Client{ctx: C.uds_init()}
}

func NewClientWithConfig(path string) *Client {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))
	return &Client{ctx: C.uds_init_with_config(cPath)}
}

func (c *Client) Close() {
	if c.ctx != nil {
		C.uds_destroy(c.ctx)
		c.ctx = nil
	}
}

func (c *Client) Discover(timeout time.Duration) ([]DeviceInfo, error) {
	var devices *C.uds_device_t
	var count C.size_t
	err := C.uds_discover(c.ctx, &devices, &count, C.uint32_t(timeout.Milliseconds()))
	if err != 0 {
		return nil, fmt.Errorf("discover failed: %d", err)
	}
	defer C.uds_free_devices(devices, count)

	result := make([]DeviceInfo, int(count))
	for i := 0; i < int(count); i++ {
		d := (*C.uds_device_t)(unsafe.Pointer(uintptr(unsafe.Pointer(devices)) + uintptr(i)*unsafe.Sizeof(*devices)))
		result[i] = DeviceInfo{
			ID:        C.GoString(d.id),
			Name:      C.GoString(d.name),
			Platform:  C.GoString(d.platform),
			Transport: C.GoString(d.transport),
			Connected: bool(d.connected != 0),
		}
	}
	return result, nil
}

func (c *Client) Inspect(deviceID string) (*DeviceInfo, error) {
	cID := C.CString(deviceID)
	defer C.free(unsafe.Pointer(cID))
	jsonStr := C.uds_inspect_device(c.ctx, cID)
	if jsonStr == nil {
		return nil, fmt.Errorf("device not found: %s", deviceID)
	}
	defer C.uds_free_string(jsonStr)
	return &DeviceInfo{ID: deviceID, Name: ""}, nil
}

func (c *Client) Flash(deviceID string, firmwarePath string) error {
	cID := C.CString(deviceID)
	cPath := C.CString(firmwarePath)
	defer C.free(unsafe.Pointer(cID))
	defer C.free(unsafe.Pointer(cPath))

	result := C.uds_flash(c.ctx, cID, cPath)
	if result != 0 {
		return fmt.Errorf("flash failed with code %d", result)
	}
	return nil
}

func (c *Client) RPC(deviceID string, method string, params string) (string, error) {
	cID := C.CString(deviceID)
	cMethod := C.CString(method)
	cParams := C.CString(params)
	defer C.free(unsafe.Pointer(cID))
	defer C.free(unsafe.Pointer(cMethod))
	defer C.free(unsafe.Pointer(cParams))

	jsonStr := C.uds_rpc(c.ctx, cID, cMethod, cParams)
	if jsonStr == nil {
		return "", fmt.Errorf("rpc failed")
	}
	defer C.uds_free_string(jsonStr)
	return C.GoString(jsonStr), nil
}
