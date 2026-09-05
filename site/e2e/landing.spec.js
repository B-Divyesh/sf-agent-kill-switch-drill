import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

function seriousAxeResults(page) {
  return new AxeBuilder({ page }).analyze().then((results) => results.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact)));
}

async function downloadText(download) {
  const stream = await download.createReadStream();
  let text = '';
  for await (const chunk of stream) text += chunk.toString();
  return text;
}

test('@claim:sample-drill Try it with sample data opens a completed isolated drill', async ({ page }) => {
  const requests = [];
  page.on('request', (request) => requests.push(request.url()));

  await page.goto('/');
  const firstAction = page.getByRole('link', { name: 'Try it with sample data' });
  const actionBox = await firstAction.boundingBox();
  expect(actionBox.y + actionBox.height).toBeLessThanOrEqual(await page.evaluate(() => innerHeight));
  await firstAction.click();
  await page.waitForURL(/\?demo=1#drill$/);
  await expect(page).toHaveTitle('Demo — Agent Kill-Switch Drill');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Test your agent stop path');
  await expect(page.getByLabel('Sample data mode')).toBeVisible();
  await expect(page.getByText('Sample profile — nothing is saved')).toBeVisible();
  await expect(page.getByText('All declared paths confirmed')).toBeVisible();
  await expect(page.locator('#checkpoint-list li')).toHaveCount(3);
  await expect(page.getByRole('button', { name: 'Export incident card' })).toBeEnabled();
  await expect.poll(() => page.evaluate(() => Object.keys(localStorage))).toContain('demo:agent-kill-switch-drill:drill');

  await page.getByRole('button', { name: 'Reset demo' }).click();
  await expect(page.getByText('All three checkpoints were simulated and confirmed. Selected: Deny proxy route.')).toBeVisible();
  await page.getByRole('button', { name: 'Start for real' }).click();
  await page.waitForURL(/\/#drill$/);
  await expect(page.getByLabel('Sample data mode')).toBeHidden();
  await expect.poll(() => page.evaluate(() => localStorage.getItem('demo:agent-kill-switch-drill:drill'))).toBeNull();

  const origin = new URL(page.url()).origin;
  expect(requests.every((url) => new URL(url).origin === origin)).toBe(true);
});

test('@claim:incident-card-export The sample drill downloads a scrubbed incident card', async ({ page }) => {
  await page.goto('/?demo=1#drill');
  const [download] = await Promise.all([
    page.waitForEvent('download'),
    page.getByRole('button', { name: 'Export incident card' }).click()
  ]);
  expect(download.suggestedFilename()).toBe('incident-card.json');
  const card = JSON.parse(await downloadText(download));
  expect(card).toMatchObject({
    schema: 'agent-kill-switch-drill/incident-card@v1',
    profile: 'payments-write',
    mode: 'dry_run',
    all_confirmed: true
  });
  expect(card.checkpoints).toHaveLength(3);
  expect(card.report_safety).not.toMatch(/provider secret|command output/i);
});

test('@claim:license-gate The worksheet is available only after a valid license result', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('button', { name: 'Download tabletop worksheet pack' })).toBeHidden();
  const directFile = await page.request.get('/tabletop-worksheet.txt');
  expect(directFile.status()).toBe(404);

  await page.route(/https:\/\/api\.sociobot\.in\/api\/v1\/products\/agent-kill-switch-drill\/verify\?license=license-for-test/, (route) => route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({ valid: true, reason: 'ok', expires_at: null })
  }));
  await page.goto('/?license=license-for-test');
  await expect(page).toHaveURL(/\/$/);
  const worksheetButton = page.getByRole('button', { name: 'Download tabletop worksheet pack' });
  await expect(worksheetButton).toBeVisible();
  await expect(worksheetButton).toBeEnabled();
  const [download] = await Promise.all([page.waitForEvent('download'), worksheetButton.click()]);
  expect(download.suggestedFilename()).toBe('tabletop-worksheet.txt');
  expect(await downloadText(download)).toContain('CAPABILITY CARD');
});

test('@claim:privacy-default Default sample use makes no third-party requests', async ({ page }) => {
  const requests = [];
  page.on('request', (request) => requests.push(request.url()));
  await page.goto('/?demo=1#drill');
  await page.getByRole('button', { name: 'Export incident card' }).click();
  const origin = new URL(page.url()).origin;
  expect(requests.every((url) => new URL(url).origin === origin)).toBe(true);
});

test('both themes have semantic structure, usable touch targets, and no serious axe findings', async ({ page }) => {
  const consoleErrors = [];
  page.on('console', (message) => { if (message.type() === 'error') consoleErrors.push(message.text()); });
  page.on('pageerror', (error) => consoleErrors.push(error.message));
  await page.goto('/');
  await expect(page.locator('html')).toHaveAttribute('lang', 'en');
  await expect(page.getByRole('main')).toHaveCount(1);
  await expect(page.getByRole('heading', { level: 1 })).toHaveCount(1);
  expect(await seriousAxeResults(page)).toEqual([]);
  const targets = await page.locator('header a, footer a, header button').evaluateAll((elements) => elements.map((element) => {
    const rectangle = element.getBoundingClientRect();
    return { width: rectangle.width, height: rectangle.height };
  }));
  expect(targets.every((target) => target.width >= 44 && target.height >= 44)).toBe(true);

  await page.getByRole('button', { name: 'Dark ink' }).click();
  await expect(page.getByRole('button', { name: 'Paper mode' })).toBeVisible();
  expect(await seriousAxeResults(page)).toEqual([]);
  expect(consoleErrors).toEqual([]);
});

test('legal and not-found pages have their own titles and accessible paths', async ({ page }) => {
  await page.goto('/privacy/');
  await expect(page).toHaveTitle('Privacy — Agent Kill-Switch Drill');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Privacy for your stop drills');
  expect(await seriousAxeResults(page)).toEqual([]);
  await page.goto('/terms/');
  await expect(page).toHaveTitle('Terms — Agent Kill-Switch Drill');
  await page.goto('/404.html');
  await expect(page).toHaveTitle('Page not found — Agent Kill-Switch Drill');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('This page does not exist');
});

test('reduced motion and phone layout remain usable', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/?demo=1#drill');
  await expect(page.locator('.hero-art img')).toHaveJSProperty('complete', true);
  expect(await page.evaluate(() => getComputedStyle(document.documentElement).scrollBehavior)).toBe('auto');
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth)).toBe(true);
  await page.getByRole('button', { name: 'Export incident card' }).focus();
  await expect(page.getByRole('button', { name: 'Export incident card' })).toBeFocused();
});
