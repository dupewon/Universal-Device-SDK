import Foundation

public class UDSClient {
    private var ctx: OpaquePointer?

    public init() {
        self.ctx = uds_init()
    }

    public init(configPath: String) {
        self.ctx = uds_init_with_config(configPath)
    }

    deinit {
        if let ctx = ctx {
            uds_destroy(ctx)
        }
    }

    public func discover(timeout: TimeInterval = 5) throws -> [UDSDevice] {
        guard let ctx = ctx else { throw UDSError.notInitialized }
        let jsonPtr = uds_list_devices(ctx)
        guard let jsonPtr = jsonPtr else { return [] }
        let json = String(cString: jsonPtr)
        uds_free_string(jsonPtr)
        guard let data = json.data(using: .utf8) else { return [] }
        let decoder = JSONDecoder()
        return (try? decoder.decode([UDSDevice].self, from: data)) ?? []
    }

    public func inspect(id: String) throws -> UDSDevice {
        guard let ctx = ctx else { throw UDSError.notInitialized }
        let jsonPtr = uds_inspect_device(ctx, id)
        guard let jsonPtr = jsonPtr else { throw UDSError.deviceNotFound }
        let json = String(cString: jsonPtr)
        uds_free_string(jsonPtr)
        guard let data = json.data(using: .utf8) else { throw UDSError.parseError }
        let decoder = JSONDecoder()
        return try decoder.decode(UDSDevice.self, from: data)
    }

    public func flash(deviceId: String, firmwarePath: String) throws {
        guard let ctx = ctx else { throw UDSError.notInitialized }
        let result = uds_flash(ctx, deviceId, firmwarePath)
        if result != 0 { throw UDSError.flashFailed }
    }

    public func rpc(deviceId: String, method: String, params: String = "{}") throws -> String {
        guard let ctx = ctx else { throw UDSError.notInitialized }
        let jsonPtr = uds_rpc(ctx, deviceId, method, params)
        guard let jsonPtr = jsonPtr else { throw UDSError.rpcFailed }
        let json = String(cString: jsonPtr)
        uds_free_string(jsonPtr)
        return json
    }

    @discardableResult
    public func connect(deviceId: String) -> Bool {
        guard let ctx = ctx else { return false }
        return uds_connect(ctx, deviceId) == 0
    }

    public func disconnect(deviceId: String) {
        guard let ctx = ctx else { return }
        uds_disconnect(ctx, deviceId)
    }
}

public enum UDSError: Error {
    case notInitialized
    case deviceNotFound
    case flashFailed
    case rpcFailed
    case parseError
}

public struct UDSDevice: Codable {
    public let id: String
    public let name: String
    public let platform: String
    public let transport: String
    public let connected: Bool
    public let firmwareVersion: String?
    public let uptimeSeconds: Int?

    enum CodingKeys: String, CodingKey {
        case id, name, platform, transport, connected
        case firmwareVersion = "firmware_version"
        case uptimeSeconds = "uptime_seconds"
    }
}

// C FFI declarations
@_silgen_name("uds_init") func uds_init() -> OpaquePointer?
@_silgen_name("uds_init_with_config") func uds_init_with_config(_ path: UnsafePointer<CChar>) -> OpaquePointer?
@_silgen_name("uds_destroy") func uds_destroy(_ ctx: OpaquePointer?)
@_silgen_name("uds_list_devices") func uds_list_devices(_ ctx: OpaquePointer?) -> UnsafeMutablePointer<CChar>?
@_silgen_name("uds_inspect_device") func uds_inspect_device(_ ctx: OpaquePointer?, _ id: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?
@_silgen_name("uds_flash") func uds_flash(_ ctx: OpaquePointer?, _ deviceId: UnsafePointer<CChar>, _ path: UnsafePointer<CChar>) -> Int32
@_silgen_name("uds_rpc") func uds_rpc(_ ctx: OpaquePointer?, _ deviceId: UnsafePointer<CChar>, _ method: UnsafePointer<CChar>, _ params: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?
@_silgen_name("uds_connect") func uds_connect(_ ctx: OpaquePointer?, _ deviceId: UnsafePointer<CChar>) -> Int32
@_silgen_name("uds_disconnect") func uds_disconnect(_ ctx: OpaquePointer?, _ deviceId: UnsafePointer<CChar>)
@_silgen_name("uds_free_string") func uds_free_string(_ ptr: UnsafeMutablePointer<CChar>?)
