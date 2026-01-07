import { test, expect } from "../fixtures/test-fixtures";
import { MarketListPage } from "../pages/market-list.page";

test.describe("Trading Panel", () => {
  test("Split tab shows correct preview (deposit X USD → receive X YES + X NO)", async ({
    page,
    api,
  }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Split preview test ${Date.now()}`;
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
        { timeout: 15000, intervals: [1000, 2000, 3000, 4000, 5000] }
      )
      .toBe(true);

    const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
    await marketCard.click();

    const tradePanel = marketCard.locator('[data-testid="trade-panel"]');
    await expect(tradePanel).toBeVisible({ timeout: 5000 });

    const splitTab = marketCard.locator('[data-testid="tab-split"]');
    await splitTab.click();

    const input = marketCard.locator('input[type="number"]');
    await input.fill("10");

    const splitInfo = marketCard.locator(".split-info");
    await expect(splitInfo).toBeVisible();
    await expect(splitInfo).toContainText("$10 USD");
    await expect(splitInfo).toContainText("10 YES + 10 NO");
  });

  test("Buy YES tab shows quote when amount entered", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Buy YES quote test ${Date.now()}`;
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
        { timeout: 15000, intervals: [1000, 2000, 3000, 4000, 5000] }
      )
      .toBe(true);

    const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
    await marketCard.click();

    const tradePanel = marketCard.locator('[data-testid="trade-panel"]');
    await expect(tradePanel).toBeVisible({ timeout: 5000 });

    const buyYesTab = marketCard.locator('[data-testid="tab-buy-yes"]');
    await expect(buyYesTab).toHaveClass(/active/);

    const input = marketCard.locator('input[type="number"]');
    await input.fill("10");

    const quoteBox = marketCard.locator(".quote-box");
    await expect(quoteBox).toBeVisible();
    await expect(quoteBox).toContainText("You pay");
  });

  test("Buy NO tab shows quote when amount entered", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Buy NO quote test ${Date.now()}`;
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
        { timeout: 15000, intervals: [1000, 2000, 3000, 4000, 5000] }
      )
      .toBe(true);

    const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
    await marketCard.click();

    const tradePanel = marketCard.locator('[data-testid="trade-panel"]');
    await expect(tradePanel).toBeVisible({ timeout: 5000 });

    const buyNoTab = marketCard.locator('[data-testid="tab-buy-no"]');
    await buyNoTab.click();
    await expect(buyNoTab).toHaveClass(/active/);

    const input = marketCard.locator('input[type="number"]');
    await input.fill("10");

    const quoteBox = marketCard.locator(".quote-box");
    await expect(quoteBox).toBeVisible();
    await expect(quoteBox).toContainText("You pay");
  });

  test('Trade button shows "Connect Wallet" when not connected', async ({
    page,
    api,
  }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Connect wallet button test ${Date.now()}`;
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
        { timeout: 15000, intervals: [1000, 2000, 3000, 4000, 5000] }
      )
      .toBe(true);

    const marketCard = marketListPage.getMarketByQuestion(uniqueQuestion);
    await marketCard.click();

    const tradePanel = marketCard.locator('[data-testid="trade-panel"]');
    await expect(tradePanel).toBeVisible({ timeout: 5000 });

    const tradeButton = tradePanel.locator("button.btn");
    await expect(tradeButton).toHaveText("Connect Wallet");
  });
});
