#pragma once

#include <stdint.h>

namespace verseguy {
    enum class Capability : uint64_t {
        StorageRead = 1ull << 0,
        StorageWrite = 1ull << 1,
        NetworkP2P = 1ull << 2,
        NetworkMaster = 1ull << 3,
        UiPanel = 1ull << 4,
        Notifications = 1ull << 5,
        FilesystemRead = 1ull << 6,
        FilesystemWrite = 1ull << 7,
    };
}
