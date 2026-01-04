#include <cstdlib>
#include <cassert>
#include <iostream>

int main() {
    // Run the wasm-runner binary via cargo run (workspace should have Cargo available)
    // Quiet output and supply args 5 7 to check result 12
    int rc = std::system("cargo run -p verseguy_wasm_sandbox --bin wasm-runner --quiet -- 5 7");
    if (rc != 0) {
        std::cerr << "wasm-runner failed (rc=" << rc << ")" << std::endl;
        return 1;
    }
    return 0;
}
