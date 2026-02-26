// Use global Tauri API (withGlobalTauri: true) — no bundler required
const invoke = window.__TAURI__.core.invoke;

const portsEl = document.getElementById('ports');
const portsErrorEl = document.getElementById('ports-error');
const refreshBtn = document.getElementById('refresh');
const mockBanner = document.getElementById('mock-banner');
const statusCard = document.getElementById('status-card');
const statusText = document.getElementById('status-text');
const statusRefreshBtn = document.getElementById('status-refresh');

async function refreshPorts() {
  portsErrorEl.hidden = true;
  portsEl.innerHTML = '';
  refreshBtn.disabled = true;
  try {
    const ports = await invoke('list_serial_ports');
    if (!ports || ports.length === 0) {
      portsEl.innerHTML = '<li>No serial ports found</li>';
    } else {
      ports.forEach((p) => {
        const li = document.createElement('li');
        li.textContent = `${p.title || p.name} (${p.name})`;
        portsEl.appendChild(li);
      });
    }
  } catch (e) {
    portsErrorEl.textContent = String(e);
    portsErrorEl.hidden = false;
  } finally {
    refreshBtn.disabled = false;
  }
}

function formatStatus(s) {
  const w = s.work_pos;
  const m = s.machine_pos;
  return `State: ${s.state}\nWork:  X${w.x.toFixed(3)} Y${w.y.toFixed(3)} Z${w.z.toFixed(3)}\nMachine: X${m.x.toFixed(3)} Y${m.y.toFixed(3)} Z${m.z.toFixed(3)}\nFeed: ${s.feed_rate} mm/min  Spindle: ${s.spindle_speed} rpm`;
}

async function refreshStatus() {
  try {
    const s = await invoke('get_mock_status');
    statusText.textContent = formatStatus(s);
  } catch (e) {
    statusText.textContent = 'Error: ' + String(e);
  }
}

async function init() {
  try {
    const mock = await invoke('is_mock_mode');
    mockBanner.hidden = !mock;
    statusCard.hidden = !mock;
    if (mock) {
      await refreshStatus();
    }
  } catch (_) {
    mockBanner.hidden = true;
    statusCard.hidden = true;
  }
  await refreshPorts();
}

refreshBtn.addEventListener('click', refreshPorts);
statusRefreshBtn.addEventListener('click', refreshStatus);
init();
