# Rust Multiplatform Experiments

Playing with tools in the Rust ecosystem such as Uniffi and wasm-bindgen to create multiplatform libraries 

## Notes

### Creating a Swift package that wraps the static library and Uniffi bindings

1. Build the uniffi-bindgen cli tool

    `cargo build -p uniffi-bindgen`

1. Compile the static library (e.g. for x86_64 simulator arch)

    `cargo build -p shared --target=x86_64-apple-ios`

1. Generate the Uniffi bindings

    `cargo run -p uniffi-bindgen generate shared/src/shared.udl --language swift --out-dir shared/src/iosBindings/`

1. Rename the generated `<GENERATED_NAME>.modulemap` to be `module.modulemap` so it's understood by xcode to be an XCFramework module map

1. Create the directory for the Swift package - `mkdir SharedPackage`

1. Initialize the Swift package

    `cd SharedPacakge && swift package init --name Shared --type library`

1. Create the XCFramework. Add as many pairs of `-library <PATH_TO_LIB> -headers <PATH_TO_HEADERS>` as architectures you are supporting.

    ```
    xcodebuild -create-xcframework \
    -library target/x86_64-apple-ios/debug/libshared.a \
    -headers shared/src/iosBindings/ \
    -output SharedPackage/sharedFFI.xcframework
    ```

1. Edit the package's source file (e.g. the `Shared.swift` file) to include the generated Swift "glue" provided by the Uniffi bindings

1. Edit the package's project file (e.g. `Package.swift`) to include the XCFramework as a `.binaryTarget`. See https://developer.apple.com/documentation/xcode/creating-a-standalone-swift-package-with-xcode and https://blog.devgenius.io/xcframework-swiftpackagemanager-c03f4b1903d9 and https://rhonabwy.com/2023/02/10/creating-an-xcframework/ for examples of how.

1. Add the XCFramework to the app project. See https://www.simpleswiftguide.com/how-to-add-xcframework-to-xcode-project/ and https://engineering.talkdesk.com/create-build-and-link-a-framework-into-an-ios-project-db05a3a26c63 for examples of how.

1. Use the library!


### Integrating with Gradle for Android

1. Following the instructions at https://redbadger.github.io/crux/getting_started/Android/android.html. Crux is a Rust framework that builds on top of the Uniffi output.

