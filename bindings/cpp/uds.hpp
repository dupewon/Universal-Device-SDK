#pragma once

#include <string>
#include <vector>
#include <memory>
#include <functional>

namespace uds {

class Device {
public:
    std::string id;
    std::string name;
    std::string transport_type;
    std::string address;

    std::string to_string() const;
};

class Context {
public:
    Context();
    ~Context();

    std::vector<Device> discover(uint32_t timeout_ms = 5000);
    void connect(const Device& device);
    void disconnect(const Device& device);

    void flash(const Device& device, const std::vector<uint8_t>& firmware);
    void monitor(const Device& device, std::function<void(const std::string&)> callback);

    std::vector<uint8_t> rpc_call(const Device& device, const std::string& method,
                                   const std::vector<uint8_t>& params);

private:
    struct Impl;
    std::unique_ptr<Impl> impl_;
};

}
