import { test, expect } from "../fixtures/test-fixtures";
import { MarketListPage } from "../pages/market-list.page";

test.describe("Market Opening", () => {
  test("admin can open a market and it appears in the list", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Will test pass? ${Date.now()}`;
    const resolutionDeadline = Math.floor(Date.now() / 1000) + 86400 * 30;

    await api.openMarket(uniqueQuestion, resolutionDeadline);

    await marketListPage.goto();

    await expect
      .poll(
        async () => {
          await page.reload();
          const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
          return await marketCard.isVisible();
        },
        { timeout: 15000, intervals: [1000, 2000, 3000] }
      )
      .toBe(true);

    const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
    await expect(marketCard.locator('[data-testid="market-question"]')).toHaveText(uniqueQuestion);
  });

  test("clicking a market card expands it to show trade panel", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Click to expand test ${Date.now()}`;
    const resolutionDeadline = Math.floor(Date.now() / 1000) + 86400 * 30;

    await api.openMarket(uniqueQuestion, resolutionDeadline);
    await marketListPage.goto();

    await expect
      .poll(
        async () => {
          await page.reload();
          const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
          return await marketCard.isVisible();
        },
        { timeout: 15000, intervals: [1000, 2000, 3000] }
      )
      .toBe(true);

    const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
    await marketCard.click();

    const tradePanel = marketCard.locator('[data-testid="trade-panel"]');
    await expect(tradePanel).toBeVisible({ timeout: 5000 });
  });

  test("expanded market shows YES/NO price boxes", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Price boxes test ${Date.now()}`;
    const resolutionDeadline = Math.floor(Date.now() / 1000) + 86400 * 30;

    await api.openMarket(uniqueQuestion, resolutionDeadline);
    await marketListPage.goto();

    await expect
      .poll(
        async () => {
          await page.reload();
          const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
          return await marketCard.isVisible();
        },
        { timeout: 15000, intervals: [1000, 2000, 3000] }
      )
      .toBe(true);

    const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
    await marketCard.click();

    const priceBoxYes = marketCard.locator('[data-testid="price-box-yes"]');
    const priceBoxNo = marketCard.locator('[data-testid="price-box-no"]');

    await expect(priceBoxYes).toBeVisible({ timeout: 5000 });
    await expect(priceBoxNo).toBeVisible();
    await expect(priceBoxYes).toContainText("YES");
    await expect(priceBoxNo).toContainText("NO");
  });

  test("expanded market shows trade tabs (Buy YES, Buy NO, Split)", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Trade tabs test ${Date.now()}`;
    const resolutionDeadline = Math.floor(Date.now() / 1000) + 86400 * 30;

    await api.openMarket(uniqueQuestion, resolutionDeadline);
    await marketListPage.goto();

    await expect
      .poll(
        async () => {
          await page.reload();
          const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
          return await marketCard.isVisible();
        },
        { timeout: 15000, intervals: [1000, 2000, 3000] }
      )
      .toBe(true);

    const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
    await marketCard.click();

    const tradeTabs = marketCard.locator('[data-testid="trade-tabs"]');
    await expect(tradeTabs).toBeVisible({ timeout: 5000 });

    const buyYesTab = marketCard.locator('[data-testid="tab-buy-yes"]');
    const buyNoTab = marketCard.locator('[data-testid="tab-buy-no"]');
    const splitTab = marketCard.locator('[data-testid="tab-split"]');

    await expect(buyYesTab).toBeVisible();
    await expect(buyNoTab).toBeVisible();
    await expect(splitTab).toBeVisible();

    await expect(buyYesTab).toHaveText("Buy YES");
    await expect(buyNoTab).toHaveText("Buy NO");
    await expect(splitTab).toHaveText("Split");
  });
});
