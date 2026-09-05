import './style.css';
import worksheet from './tabletop-worksheet.txt?raw';

const stages = [
  ['Deny proxy route', 'HTTP 403 returned by the declared proxy check.'],
  ['Revoke credential', 'Credential status returned revoked.'],
  ['Pause queue', 'Queue status returned paused.']
];
const product = 'agent-kill-switch-drill';
const demoStorageKey = `demo:${product}:drill`;
const licenseKey = `sb_license:${product}`;
const verdictKey = `${licenseKey}:verdict`;
const $ = (selector) => document.querySelector(selector);
const state = { selected: 0, ran: false, demo: false };

function readStored(key) {
  try { return JSON.parse(localStorage.getItem(key) || 'null'); } catch { return null; }
}

function saveDemo() {
  if (state.demo) localStorage.setItem(demoStorageKey, JSON.stringify({ selected: state.selected, ran: state.ran }));
}

function render() {
  document.querySelectorAll('.stage').forEach((button, index) => button.classList.toggle('active', index === state.selected));
  $('#stage-summary').textContent = state.ran
    ? `All three checkpoints were simulated and confirmed. Selected: ${stages[state.selected][0]}.`
    : `Checkpoint ${String(state.selected + 1).padStart(2, '0')} is selected. Run the simulation to record all three checks.`;
  $('#card-stamp').textContent = state.ran ? 'CONFIRMED' : 'READY';
  $('#result-text').textContent = state.ran ? 'All declared paths confirmed' : 'Not yet run';
  $('#export-card').disabled = !state.ran;
  $('#sample-label').hidden = !state.demo;
  $('#demo-banner').hidden = !state.demo;
  $('#checkpoint-list').innerHTML = stages.map(([name, verification], index) => `<li class="${state.ran ? 'passed' : index === state.selected ? 'selected' : ''}"><span>${String(index + 1).padStart(2, '0')}</span><div><strong>${name}</strong><small>${state.ran ? `✓ Action simulated · ${verification}` : 'Action simulated · awaiting verification'}</small></div></li>`).join('');
}

function resetDemo() {
  state.demo = true;
  state.selected = 0;
  state.ran = true;
  saveDemo();
  render();
}

function beginDemoFromUrl() {
  if (new URLSearchParams(location.search).get('demo') !== '1') return;
  const saved = readStored(demoStorageKey);
  state.demo = true;
  state.selected = Number.isInteger(saved?.selected) && saved.selected >= 0 && saved.selected < stages.length ? saved.selected : 0;
  state.ran = true;
  saveDemo();
  document.title = 'Demo — Agent Kill-Switch Drill';
}

function download(filename, content, type) {
  const url = URL.createObjectURL(new Blob([content], { type }));
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

function exportCard() {
  const card = {
    schema: 'agent-kill-switch-drill/incident-card@v1',
    profile: 'payments-write',
    mode: 'dry_run',
    all_confirmed: true,
    checkpoints: stages.map(([name, verification], index) => ({
      name,
      action_id: ['proxy_deny', 'credential_revoke', 'queue_pause'][index],
      action: 'simulated',
      verification: 'passed',
      note: verification
    })),
    report_safety: 'Command IDs and statuses only. No commands, output, or secrets.'
  };
  download('incident-card.json', JSON.stringify(card, null, 2), 'application/json');
}

document.querySelectorAll('.stage').forEach((button) => button.addEventListener('click', () => {
  state.selected = Number(button.dataset.stage);
  saveDemo();
  render();
}));
$('#run-drill').addEventListener('click', () => { state.ran = true; saveDemo(); render(); $('#run-drill').textContent = 'Simulation recorded — run again'; });
$('#export-card').addEventListener('click', exportCard);
$('#reset-demo').addEventListener('click', resetDemo);
$('#start-real').addEventListener('click', () => { localStorage.removeItem(demoStorageKey); location.assign(`${location.pathname}${location.hash}`); });

const theme = localStorage.getItem('aksd-theme');
if (theme === 'dark') document.documentElement.dataset.theme = 'dark';
function updateTheme() {
  const dark = document.documentElement.dataset.theme === 'dark';
  $('#theme-toggle').setAttribute('aria-pressed', String(dark));
  $('#theme-toggle').textContent = dark ? 'Paper mode' : 'Dark ink';
}
$('#theme-toggle').addEventListener('click', () => {
  document.documentElement.dataset.theme = document.documentElement.dataset.theme === 'dark' ? '' : 'dark';
  localStorage.setItem('aksd-theme', document.documentElement.dataset.theme || 'light');
  updateTheme();
});
updateTheme();

const status = $('#license-status');
function showLicense(message, kind = '') { status.textContent = message; status.dataset.kind = kind; }
function setUnlocked(unlocked) {
  $('#worksheet-download').hidden = !unlocked;
  $('#worksheet-download').disabled = !unlocked;
}
function tokenFromUrl() {
  const url = new URL(location.href);
  const token = url.searchParams.get('license');
  if (token) {
    localStorage.setItem(licenseKey, token);
    url.searchParams.delete('license');
    history.replaceState({}, '', `${url.pathname}${url.search}${url.hash}`);
  }
  return token;
}
async function verifyLicense(token) {
  const cached = readStored(verdictKey);
  const cachedForToken = cached?.token === token ? cached : null;
  const fresh = cachedForToken && Date.now() - cachedForToken.at < 86400000;
  if (fresh) {
    setUnlocked(cachedForToken.valid);
    showLicense(cachedForToken.valid ? 'Support unlock active.' : 'License no longer active.', cachedForToken.valid ? 'ok' : 'error');
    return cachedForToken.valid;
  }
  if (!navigator.onLine) {
    setUnlocked(Boolean(cachedForToken?.valid));
    showLicense(cachedForToken?.valid ? 'Support unlock active (offline cache).' : 'Offline — license will verify when connected.', cachedForToken?.valid ? 'ok' : 'warn');
    return Boolean(cachedForToken?.valid);
  }
  showLicense('Checking license…');
  try {
    const response = await fetch(`https://api.sociobot.in/api/v1/products/${product}/verify?license=${encodeURIComponent(token)}`, { cache: 'no-store' });
    if (!response.ok) throw new Error('license verification failed');
    const data = await response.json();
    const value = { token, valid: data.valid === true, at: Date.now() };
    localStorage.setItem(verdictKey, JSON.stringify(value));
    setUnlocked(value.valid);
    showLicense(value.valid ? 'Support unlock active.' : 'License no longer active.', value.valid ? 'ok' : 'error');
    return value.valid;
  } catch {
    setUnlocked(false);
    showLicense('Could not verify now. Your free tools still work.', 'warn');
    return false;
  }
}
const returnedToken = tokenFromUrl();
const storedToken = returnedToken || localStorage.getItem(licenseKey);
const initialVerdict = readStored(verdictKey);
if (initialVerdict?.token === storedToken && initialVerdict.valid) setUnlocked(true);
if (storedToken) verifyLicense(storedToken);
$('#license-form').addEventListener('submit', (event) => {
  event.preventDefault();
  const token = $('#license-input').value.trim();
  if (!token) { showLicense('Paste a license token to restore it.', 'error'); return; }
  localStorage.setItem(licenseKey, token);
  verifyLicense(token);
});
$('#worksheet-download').addEventListener('click', () => download('tabletop-worksheet.txt', worksheet, 'text/plain'));
document.querySelectorAll('.copy').forEach((button) => button.addEventListener('click', async () => {
  try {
    await navigator.clipboard.writeText(button.dataset.copy);
    const old = button.textContent;
    button.textContent = 'Copied';
    setTimeout(() => { button.textContent = old; }, 1500);
  } catch { button.textContent = 'Copy unavailable'; }
}));

beginDemoFromUrl();
render();
