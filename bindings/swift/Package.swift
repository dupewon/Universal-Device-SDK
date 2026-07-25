// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: "UDS",
    products: [
        .library(name: "UDS", targets: ["UDS"]),
    ],
    targets: [
        .target(
            name: "UDS",
            dependencies: ["CUDS"]
        ),
        .systemLibrary(
            name: "CUDS",
            path: "Sources/CUDS",
            pkgConfig: "uds_c"
        ),
    ]
)
