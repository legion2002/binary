import { test, expect } from "../fixtures/test-fixtures";
import { MarketListPage } from "../pages/market-list.page";

test.describe("Probability Display", () => {
  test("new market shows approximately 50% probability", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `50% probability test ${Date.now()}`;
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

    const probabilityElement = marketCard.locator('[data-testid="market-probability"]');
    await expect(probabilityElement).toBeVisible({ timeout: 5000 });

    const probabilityText = await probabilityElement.textContent();
    const probability = parseInt(probabilityText?.replace("%", "") || "0", 10);

    expect(probability).toBeGreaterThanOrEqual(45);
    expect(probability).toBeLessThanOrEqual(55);
  });

  test("YES and NO prices shown in price grid", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const uniqueQuestion = `Price grid test ${Date.now()}`;
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

    const priceGrid = marketCard.locator('[data-testid="price-grid"]');
    await expect(priceGrid).toBeVisible({ timeout: 5000 });

    const priceBoxYes = marketCard.locator('[data-testid="price-box-yes"]');
    const priceBoxNo = marketCard.locator('[data-testid="price-box-no"]');

    await expect(priceBoxYes).toBeVisible();
    await expect(priceBoxNo).toBeVisible();

    await expect(priceBoxYes.locator(".price-value")).toBeVisible();
    await expect(priceBoxNo.locator(".price-value")).toBeVisible();
  });
});
