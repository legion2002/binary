import { test, expect } from "../fixtures/test-fixtures";
import { MarketListPage } from "../pages/market-list.page";

test.describe("Multi-Market Display", () => {
  test("multiple markets are listed correctly", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const timestamp = Date.now();
    const questions = [
      `Multi-market test A ${timestamp}`,
      `Multi-market test B ${timestamp}`,
      `Multi-market test C ${timestamp}`,
    ];
    const resolutionDeadline = Math.floor(Date.now() / 1000) + 86400 * 30;

    for (const question of questions) {
      await api.openMarket(question, resolutionDeadline);
    }

    await marketListPage.goto();

    await expect
      .poll(
        async () => {
          await page.reload();
          const allVisible = await Promise.all(
            questions.map(async (q) => {
              const card = marketListPage.getMarketByQuestion(q);
              return await card.isVisible();
            })
          );
          return allVisible.every(Boolean);
        },
        { timeout: 20000, intervals: [1000, 2000, 3000, 4000, 5000] }
      )
      .toBe(true);

    for (const question of questions) {
      const card = marketListPage.getMarketByQuestion(question);
      await expect(card).toBeVisible();
    }

    const marketsCount = marketListPage.getMarketsCount();
    const countText = await marketsCount.textContent();
    const match = countText?.match(/(\d+)/);
    const count = match ? parseInt(match[1], 10) : 0;
    expect(count).toBeGreaterThanOrEqual(3);
  });

  test("markets can be expanded independently", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const timestamp = Date.now();
    const question1 = `Independent expand A ${timestamp}`;
    const question2 = `Independent expand B ${timestamp}`;
    const resolutionDeadline = Math.floor(Date.now() / 1000) + 86400 * 30;

    await api.openMarket(question1, resolutionDeadline);
    await api.openMarket(question2, resolutionDeadline);

    await marketListPage.goto();

    await expect
      .poll(
        async () => {
          await page.reload();
          const card1 = marketListPage.getMarketByQuestion(question1);
          const card2 = marketListPage.getMarketByQuestion(question2);
          return (await card1.isVisible()) && (await card2.isVisible());
        },
        { timeout: 20000, intervals: [1000, 2000, 3000, 4000, 5000] }
      )
      .toBe(true);

    const card1 = marketListPage.getMarketByQuestion(question1);
    const card2 = marketListPage.getMarketByQuestion(question2);

    await card1.click();
    const tradePanel1 = card1.locator('[data-testid="trade-panel"]');
    await expect(tradePanel1).toBeVisible({ timeout: 5000 });

    const tradePanel2Before = card2.locator('[data-testid="trade-panel"]');
    await expect(tradePanel2Before).not.toBeVisible();

    await card2.click();
    const tradePanel2 = card2.locator('[data-testid="trade-panel"]');
    await expect(tradePanel2).toBeVisible({ timeout: 5000 });

    await expect(tradePanel1).toBeVisible();
    await expect(tradePanel2).toBeVisible();
  });

  test("each market shows its own question", async ({ page, api }) => {
    const marketListPage = new MarketListPage(page);
    const timestamp = Date.now();
    const questions = [
      `Unique question Alpha ${timestamp}`,
      `Unique question Beta ${timestamp}`,
      `Unique question Gamma ${timestamp}`,
    ];
    const resolutionDeadline = Math.floor(Date.now() / 1000) + 86400 * 30;

    for (const question of questions) {
      await api.openMarket(question, resolutionDeadline);
    }

    await marketListPage.goto();

    await expect
      .poll(
        async () => {
          await page.reload();
          const allVisible = await Promise.all(
            questions.map(async (q) => {
              const card = marketListPage.getMarketByQuestion(q);
              return await card.isVisible();
            })
          );
          return allVisible.every(Boolean);
        },
        { timeout: 20000, intervals: [1000, 2000, 3000, 4000, 5000] }
      )
      .toBe(true);

    for (const question of questions) {
      const card = marketListPage.getMarketByQuestion(question);
      const questionEl = card.locator('[data-testid="market-question"]');
      await expect(questionEl).toHaveText(question);
    }
  });
});
