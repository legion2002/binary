import { test as base, expect } from "@playwright/test";
import { TestApiClient } from "./api-helpers";

type TestFixtures = {
  api: TestApiClient;
};

export const test = base.extend<TestFixtures>({
  api: async (_deps, use) => {
    const api = new TestApiClient();
    // eslint-disable-next-line react-hooks/rules-of-hooks
    await use(api);
  },
});

export { expect };
