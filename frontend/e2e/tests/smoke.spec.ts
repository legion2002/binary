import { test, expect } from "../fixtures/test-fixtures";
import { MarketListPage } from "../pages/market-list.page";

test.describe("Smoke Tests", () => {
  test("app loads successfully with Binary logo in header", async ({ page }) => {
    await page.goto("/");
    
    const logo = page.locator('[data-testid="logo"]');
    await expect(logo).toBeVisible();
    await expect(logo).toHaveText("Binary");
  });

  test("header shows Tempo badge", async ({ page }) => {
    await page.goto("/");
    
    const tempoBadge = page.locator('[data-testid="tempo-badge"]');
    await expect(tempoBadge).toBeVisible();
    await expect(tempoBadge).toContainText("Tempo");
  });

  test("Connect button is visible", async ({ page }) => {
    await page.goto("/");
    
    const connectButton = page.locator('[data-testid="connect-button"]');
    await expect(connectButton).toBeVisible();
    await expect(connectButton).toHaveText("Connect");
  });

  test("markets section is visible (either with markets or empty state)", async ({ page }) => {
    const marketListPage = new MarketListPage(page);
    await marketListPage.goto();
    
    const marketsSection = page.locator('[data-testid="markets-section"]');
    const emptyState = marketListPage.getEmptyState();
    
    await expect(marketsSection.or(emptyState)).toBeVisible({ timeout: 10000 });
  });
});
