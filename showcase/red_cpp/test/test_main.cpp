#include <gtest/gtest.h>

#include <iostream>

#include "../include/defs.h"

TEST(TestDefs, TestDefsString) {
  EXPECT_EQ(Defs::gabbro, "Gabbro");
  EXPECT_EQ(Defs::marble, "Marble");
  EXPECT_EQ(Defs::metamorphic, "Metamorphic");
}

TEST(TestDefs, TestDefsLong) {
  EXPECT_EQ(Defs::apples_count, 236);
  EXPECT_EQ(Defs::oranges_count, 454588979794318);
}