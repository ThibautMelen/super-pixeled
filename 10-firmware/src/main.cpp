#include <iostream>
#include "renderer.hpp"

int main() {
    superpixeled::Renderer renderer(64, 32);

    std::cout << "Super Pixeled Firmware v0.1.0\n";
    std::cout << "Panel: " << renderer.width() << "x" << renderer.height();
    std::cout << " (" << renderer.pixel_count() << " pixels)\n";

    if (renderer.is_healthy()) {
        std::cout << "Status: OK\n";
        return 0;
    }

    std::cerr << "Status: ERROR\n";
    return 1;
}
