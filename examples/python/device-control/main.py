from uds import UdsClient, Device
import time

def main():
    client = UdsClient()

    devices = client.discover()
    if not devices:
        print("No devices found")
        return

    dev = devices[0]
    print(f"Found device: {dev.name} ({dev.id})")

    print("Monitoring device logs...")
    client.monitor(dev.id)

if __name__ == "__main__":
    main()
