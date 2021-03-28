// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "slox",
    dependencies: [],
    targets: [
        .target(
            name: "slox",
            dependencies: []),
        .testTarget(
            name: "sloxTests",
            dependencies: ["slox"]),
    ]
)
