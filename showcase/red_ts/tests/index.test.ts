import { describe, expect, test } from "@jest/globals";

import { Defs } from "../src/index";

describe("testing Defs", () => {
  test("checking strings", () => {
    expect(Defs.GABBRO).toBe("Gabbro");
    expect(Defs.MARBLE).toBe("Marble");
    expect(Defs.METAMORPHIC).toBe("Metamorphic");
  });
  test("checking numbers", () => {
    expect(Defs.APPLES_COUNT).toBe(236);
    expect(Defs.ORANGES_COUNT).toBe(454588979794318);
  });
});
