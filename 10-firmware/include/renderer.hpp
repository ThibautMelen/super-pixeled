#pragma once

#include <cstdint>
#include <string>

namespace superpixeled {

/// LED Panel renderer class
class Renderer {
public:
    Renderer(int width, int height);
    ~Renderer() = default;

    /// Get panel width in pixels
    [[nodiscard]] int width() const noexcept { return width_; }

    /// Get panel height in pixels
    [[nodiscard]] int height() const noexcept { return height_; }

    /// Get total pixel count
    [[nodiscard]] int pixel_count() const noexcept { return width_ * height_; }

    /// Clear the framebuffer
    void clear();

    /// Check if renderer is healthy
    [[nodiscard]] bool is_healthy() const noexcept { return healthy_; }

private:
    int width_;
    int height_;
    bool healthy_ = true;
};

}  // namespace superpixeled
