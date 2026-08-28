import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './site/e2e',
  use: { baseURL: 'http://127.0.0.1:4174', browserName: 'chromium' },
  webServer: { command: 'npm run dev -- --port 4174', url: 'http://127.0.0.1:4174', reuseExistingServer: false },
  projects: [{ name: 'desktop', use: { ...devices['Desktop Chrome'] } }, { name: 'mobile', use: { ...devices['Pixel 5'] } }]
});
