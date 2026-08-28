import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('landing page has semantic essentials and the paid unlock contract', async () => {
  const html = await readFile(new URL('../index.html', import.meta.url), 'utf8');
  assert.match(html, /<html lang="en">/);
  assert.match(html, /<main id="main">/);
  assert.equal((html.match(/<h1/g) || []).length, 1);
  assert.match(html, /products\/agent-kill-switch-drill\/checkout/);
  assert.match(html, /id="license-input"/);
  assert.match(html, /id="worksheet-download"/);
});

test('site behavior keeps the license locally and exports a scrubbed card', async () => {
  const source = await readFile(new URL('../src/main.js', import.meta.url), 'utf8');
  assert.match(source, /sb_license:/);
  assert.match(source, /history\.replaceState/);
  assert.match(source, /report_safety/);
  assert.match(source, /setUnlocked/);
  assert.doesNotMatch(source, /analytics/i);
});
