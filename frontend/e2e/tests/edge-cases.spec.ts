import { test, expect } from "../fixtures/test-fixtures";
import { MarketListPage } from "../pages/market-list.page";

test.describe("Edge Cases", () => {
  test("empty amount keeps trade button disabled", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Empty amount test ${Date.now()}`;
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

    const input = marketCard.locator('input[type="number"]');
    await input.clear();

    const tradeButton = tradePanel.locator("button.btn");
    await expect(tradeButton).toBeDisabled();
  });

  test("zero amount keeps trade button disabled", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Zero amount test ${Date.now()}`;
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

    const input = marketCard.locator('input[type="number"]');
    await input.fill("0");

    const tradeButton = tradePanel.locator("button.btn");
    await expect(tradeButton).toBeDisabled();
  });

  test("negative amount is not accepted or button disabled", async ({
    page,
    api,
  }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Negative amount test ${Date.now()}`;
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

    const input = marketCard.locator('input[type="number"]');
    await input.fill("-10");

    const tradeButton = tradePanel.locator("button.btn");
    const inputValue = await input.inputValue();
    const isNegativeBlocked = inputValue === "" || inputValue === "10";
    const isButtonDisabled = await tradeButton.isDisabled();

    expect(isNegativeBlocked || isButtonDisabled).toBe(true);
  });
});
