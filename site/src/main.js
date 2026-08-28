import './style.css';

const stages = [
  ['Deny proxy route', 'HTTP 403 returned by the declared proxy check.'],
  ['Revoke credential', 'Credential status returned revoked.'],
  ['Pause queue', 'Queue status returned paused.']
];
const $ = (selector) => document.querySelector(selector);
const state = { selected: 0, ran: false };

function render() {
  document.querySelectorAll('.stage').forEach((button, index) => button.classList.toggle('active', index === state.selected));
  $('#stage-summary').textContent = state.ran ? `All three checkpoints were simulated and confirmed. Selected: ${stages[state.selected][0]}.` : `Stage ${String(state.selected + 1).padStart(2, '0')} is selected. Run the drill to record the staged confirmations.`;
  $('#card-stamp').textContent = state.ran ? 'CONFIRMED' : 'READY';
  $('#result-text').textContent = state.ran ? 'All declared paths confirmed' : 'Not yet run';
  $('#export-card').disabled = !state.ran;
  $('#checkpoint-list').innerHTML = stages.map(([name, verification], index) => `<li class="${state.ran ? 'passed' : index === state.selected ? 'selected' : ''}"><span>${String(index + 1).padStart(2, '0')}</span><div><strong>${name}</strong><small>${state.ran ? `✓ Action simulated · ${verification}` : 'Action simulated · awaiting verification'}</small></div></li>`).join('');
}

document.querySelectorAll('.stage').forEach((button) => button.addEventListener('click', () => { state.selected = Number(button.dataset.stage); render(); }));
$('#run-drill').addEventListener('click', () => { state.ran = true; render(); $('#run-drill').textContent = 'Drill recorded — run again'; });
$('#export-card').addEventListener('click', () => {
  const card = { schema: 'agent-kill-switch-drill/incident-card@v1', profile: 'payments-write', mode: 'dry_run', all_confirmed: true, checkpoints: stages.map(([name, verification], index) => ({ name, action_id: ['proxy_deny', 'credential_revoke', 'queue_pause'][index], action: 'simulated', verification: 'passed', note: verification })), report_safety: 'Command IDs and statuses only. No commands, output, or secrets.' };
  const url = URL.createObjectURL(new Blob([JSON.stringify(card, null, 2)], { type: 'application/json' })); const a = document.createElement('a'); a.href = url; a.download = 'incident-card.json'; a.click(); URL.revokeObjectURL(url);
});

const theme = localStorage.getItem('aksd-theme');
if (theme === 'dark') document.documentElement.dataset.theme = 'dark';
function updateTheme() { const dark = document.documentElement.dataset.theme === 'dark'; $('#theme-toggle').setAttribute('aria-pressed', String(dark)); $('#theme-toggle').textContent = dark ? 'Paper mode' : 'Dark ink'; }
$('#theme-toggle').addEventListener('click', () => { document.documentElement.dataset.theme = document.documentElement.dataset.theme === 'dark' ? '' : 'dark'; localStorage.setItem('aksd-theme', document.documentElement.dataset.theme || 'light'); updateTheme(); }); updateTheme();

const product = 'agent-kill-switch-drill'; const licenseKey = `sb_license:${product}`; const verdictKey = `${licenseKey}:verdict`; const status = $('#license-status');
function showLicense(message, kind = '') { status.textContent = message; status.dataset.kind = kind; }
function setUnlocked(unlocked) { $('#worksheet-download').hidden = !unlocked; }
function tokenFromUrl() { const token = new URLSearchParams(location.search).get('license'); if (token) { localStorage.setItem(licenseKey, token); history.replaceState({}, '', `${location.pathname}${location.hash}`); } return token; }
async function verifyLicense(token) {
  const cached = JSON.parse(localStorage.getItem(verdictKey) || 'null'); const fresh = cached && cached.token === token && Date.now() - cached.at < 86400000;
  if (fresh) { setUnlocked(cached.valid); showLicense(cached.valid ? 'Support unlock active.' : 'License no longer active.', cached.valid ? 'ok' : 'error'); return cached.valid; }
  if (!navigator.onLine) { setUnlocked(!!cached?.valid); showLicense(cached?.valid ? 'Support unlock active (offline cache).' : 'Offline — license will verify when connected.', cached?.valid ? 'ok' : 'warn'); return !!cached?.valid; }
  showLicense('Checking license…');
  try { const response = await fetch(`https://api.sociobot.in/api/v1/products/${product}/verify?license=${encodeURIComponent(token)}`); const data = await response.json(); const value = { token, valid: data.valid === true, at: Date.now() }; localStorage.setItem(verdictKey, JSON.stringify(value)); setUnlocked(value.valid); showLicense(value.valid ? 'Support unlock active.' : 'License no longer active.', value.valid ? 'ok' : 'error'); return value.valid; }
  catch { setUnlocked(false); showLicense('Could not verify now. Your free tools still work.', 'warn'); return false; }
}
const returnedToken = tokenFromUrl(); const storedToken = returnedToken || localStorage.getItem(licenseKey); const initialVerdict = JSON.parse(localStorage.getItem(verdictKey) || 'null'); if (initialVerdict?.token === storedToken && initialVerdict.valid) setUnlocked(true); if (storedToken) verifyLicense(storedToken);
$('#license-form').addEventListener('submit', (event) => { event.preventDefault(); const token = $('#license-input').value.trim(); if (!token) { showLicense('Paste a license token to restore it.', 'error'); return; } localStorage.setItem(licenseKey, token); verifyLicense(token); });
document.querySelectorAll('.copy').forEach((button) => button.addEventListener('click', async () => { await navigator.clipboard.writeText(button.dataset.copy); const old = button.textContent; button.textContent = 'Copied'; setTimeout(() => { button.textContent = old; }, 1500); }));
render();
