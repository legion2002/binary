import { test as base, expect } from "@playwright/test";
import { TestApiClient } from "./api-helpers";

type TestFixtures = {
  api: TestApiClient;
};

export const test = base.extend<TestFixtures>({
  api: async ({}, use) => {
    const api = new TestApiClient();
    await use(api);
  },
});

export { expect };
