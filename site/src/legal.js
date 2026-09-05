import './style.css';

const theme = localStorage.getItem('aksd-theme');
if (theme === 'dark') document.documentElement.dataset.theme = 'dark';

const toggle = document.querySelector('#theme-toggle');
function updateTheme() {
  const dark = document.documentElement.dataset.theme === 'dark';
  toggle.setAttribute('aria-pressed', String(dark));
  toggle.textContent = dark ? 'Paper mode' : 'Dark ink';
}
toggle.addEventListener('click', () => {
  document.documentElement.dataset.theme = document.documentElement.dataset.theme === 'dark' ? '' : 'dark';
  localStorage.setItem('aksd-theme', document.documentElement.dataset.theme || 'light');
  updateTheme();
});
updateTheme();
