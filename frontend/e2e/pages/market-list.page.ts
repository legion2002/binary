import { Page, Locator } from "@playwright/test";

export class MarketListPage {
  private page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  async goto(): Promise<void> {
    await this.page.goto("/");
  }

  getMarketCards(): Locator {
    return this.page.locator('[data-testid="market-card"]');
  }

  getMarketByQuestion(question: string): Locator {
    return this.page.locator('[data-testid="market-card"]', { hasText: question });
  }

  getEmptyState(): Locator {
    return this.page.locator('[data-testid="empty-state"]');
  }

  getMarketsCount(): Locator {
    return this.page.locator('[data-testid="markets-count"]');
  }
}
