import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('safe drill exports a confirmed card and has no serious accessibility issues', async ({ page }) => {
  const consoleErrors = [];
  page.on('console', (message) => { if (message.type() === 'error') consoleErrors.push(message.text()); });
  page.on('pageerror', (error) => consoleErrors.push(error.message));
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveCount(1);
  await page.getByRole('button', { name: 'Simulate this stop path' }).click();
  await expect(page.getByText('All declared paths confirmed')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Export incident card' })).toBeEnabled();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((v) => ['serious', 'critical'].includes(v.impact))).toEqual([]);
  expect(consoleErrors).toEqual([]);
});

test('live confirmation language and legal pages are reachable', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByText('Live requires friction.')).toBeVisible();
  await page.getByRole('link', { name: 'Privacy' }).click();
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Local by default.');
});
