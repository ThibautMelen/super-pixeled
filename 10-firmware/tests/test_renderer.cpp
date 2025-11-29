#include <gtest/gtest.h>
#include "renderer.hpp"

using namespace superpixeled;

TEST(RendererTest, InitializesWithCorrectDimensions) {
    Renderer renderer(64, 32);

    EXPECT_EQ(renderer.width(), 64);
    EXPECT_EQ(renderer.height(), 32);
}

TEST(RendererTest, CalculatesPixelCount) {
    Renderer renderer(64, 32);

    EXPECT_EQ(renderer.pixel_count(), 2048);
}

TEST(RendererTest, IsHealthyByDefault) {
    Renderer renderer(64, 32);

    EXPECT_TRUE(renderer.is_healthy());
}
