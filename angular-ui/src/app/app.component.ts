import { Component, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TauriService, MockStatusDto } from './core';
import { HeaderComponent, MachineCoordinatesPanelComponent, GcodeVisualizerComponent, JogControlsComponent, RunJobPanelComponent, JobConfigTabsComponent, FileLoadComponent, MainMenuComponent } from './components';
import type { MainMenuTabId } from './components';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, HeaderComponent, MachineCoordinatesPanelComponent, GcodeVisualizerComponent, JogControlsComponent, RunJobPanelComponent, JobConfigTabsComponent, FileLoadComponent, MainMenuComponent],
  template: `
    <app-header></app-header>

    <div class="app-layout">
      <app-main-menu
        [activeTab]="activeMenuTab()"
        (tabChange)="activeMenuTab.set($event)"
      ></app-main-menu>
      <main class="main-content">
        @switch (activeMenuTab()) {
          @case ('carve') {
            <div class="main-body">
              <div class="main-body-left">
                @if (mockMode) {
                  <div class="card" id="mock-banner">
                    <h2>Demo mode</h2>
                    <p style="margin:0; font-size: 0.9rem;">
                      Running with <code>MESHFORGE_MOCK=1</code>. Ports and status are fake — no machine connected.
                    </p>
                  </div>
                }

                @if (mockMode) {
                  <div class="card" id="status-card">
                    <h2>Machine status (mock)</h2>
                    <p class="status-text">{{ statusText }}</p>
                    <button type="button" (click)="refreshStatus()" style="margin-top: 0.5rem;">Refresh status</button>
                  </div>
                }
              </div>
              <div class="main-body-center">
                <app-gcode-visualizer></app-gcode-visualizer>
              </div>
              <aside class="main-body-right">
                <app-machine-coordinates-panel></app-machine-coordinates-panel>
                <app-jog-controls></app-jog-controls>
              </aside>
            </div>
            <div class="main-body-bottom">
              <div class="bottom-panel bottom-left">
                <app-file-load></app-file-load>
              </div>
              <div class="bottom-panel bottom-center">
                <app-run-job-panel></app-run-job-panel>
              </div>
              <div class="bottom-panel bottom-right">
                <app-job-config-tabs></app-job-config-tabs>
              </div>
            </div>
          }
          @case ('helper') {
            <div class="menu-placeholder">Helper — coming soon</div>
          }
          @case ('stats') {
            <div class="menu-placeholder">Stats — coming soon</div>
          }
          @case ('tools') {
            <div class="menu-placeholder">Tools — coming soon</div>
          }
          @case ('config') {
            <div class="menu-placeholder">Config — coming soon</div>
          }
        }
      </main>
    </div>
  `,
  styles: [`
    .app-layout {
      display: flex;
      flex-direction: row;
      height: calc(100vh - 48px);
    }
    .main-content {
      flex: 1;
      min-width: 0;
      padding: 1rem 1.25rem;
      display: flex;
      flex-direction: column;
    }
    .menu-placeholder {
      flex: 1;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--muted, #565f89);
      font-size: 1rem;
    }
    .main-body {
      display: flex;
      flex-direction: row;
      gap: 1.5rem;
      align-items: stretch;
      flex: 1;
      min-height: 320px;
    }
    .main-body-left {
      flex: 0 0 auto;
      min-width: 0;
    }
    .main-body-center {
      flex: 1;
      min-width: 0;
      min-height: 320px;
    }
    .main-body-right {
      flex-shrink: 0;
      display: flex;
      flex-direction: column;
      gap: 1rem;
    }
    .main-body-bottom {
      display: flex;
      flex-direction: row;
      gap: 1rem;
      margin-top: 1rem;
      align-items: stretch;
    }
    .bottom-panel {
      flex: 1;
      min-width: 0;
    }
    .bottom-left { flex: 1; }
    .bottom-center { flex: 1; }
    .bottom-right { flex: 1; }
  `],
})
export class AppComponent implements OnInit {
  readonly activeMenuTab = signal<MainMenuTabId>('carve');
  mockMode = false;
  statusText = '';

  constructor(private tauri: TauriService) {}

  ngOnInit(): void {
    this.tauri.isMockMode().then((m) => {
      this.mockMode = m;
      if (m) this.refreshStatus();
    }).catch(() => { this.mockMode = false; });
  }

  refreshStatus(): void {
    this.tauri.getMockStatus()
      .then((s) => { this.statusText = this.formatStatus(s); })
      .catch((e) => { this.statusText = 'Error: ' + String(e); });
  }

  private formatStatus(s: MockStatusDto): string {
    const w = s.work_pos;
    const m = s.machine_pos;
    return `State: ${s.state}\nWork:  X${w.x.toFixed(3)} Y${w.y.toFixed(3)} Z${w.z.toFixed(3)}\nMachine: X${m.x.toFixed(3)} Y${m.y.toFixed(3)} Z${m.z.toFixed(3)}\nFeed: ${s.feed_rate} mm/min  Spindle: ${s.spindle_speed} rpm`;
  }
}
