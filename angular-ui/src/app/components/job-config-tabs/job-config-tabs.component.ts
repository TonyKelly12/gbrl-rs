import { Component, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ProbeTabComponent } from '../probe-tab/probe-tab.component';

export type JobConfigTabId = 'probe' | 'macros' | 'spindle' | 'coolant' | 'console';

const TABS: { id: JobConfigTabId; label: string }[] = [
  { id: 'probe', label: 'Probe' },
  { id: 'macros', label: 'Macros' },
  { id: 'spindle', label: 'Spindle/Laser' },
  { id: 'coolant', label: 'Coolant' },
  { id: 'console', label: 'Console' },
];

@Component({
  selector: 'app-job-config-tabs',
  standalone: true,
  imports: [CommonModule, ProbeTabComponent],
  template: `
    <section class="job-config-tabs" aria-label="Job configuration">
      <div class="tabs-header">
        <button type="button" class="tab-scroll tab-scroll-left" aria-label="Scroll tabs left">&#9664;</button>
        <nav class="tabs-nav" role="tablist">
          @for (tab of tabs; track tab.id) {
            <button
              type="button"
              class="tab"
              [class.tab-active]="activeTab() === tab.id"
              (click)="activeTab.set(tab.id)"
              [attr.aria-selected]="activeTab() === tab.id"
              [attr.aria-label]="tab.label"
              role="tab"
            >
              {{ tab.label }}
            </button>
          }
        </nav>
        <button type="button" class="tab-scroll tab-scroll-right" aria-label="Scroll tabs right">&#9654;</button>
      </div>
      <div class="tabs-content" role="tabpanel">
        @switch (activeTab()) {
          @case ('probe') {
            <app-probe-tab></app-probe-tab>
          }
          @case ('macros') {
            <div class="tab-placeholder">Macros — coming soon</div>
          }
          @case ('spindle') {
            <div class="tab-placeholder">Spindle/Laser — coming soon</div>
          }
          @case ('coolant') {
            <div class="tab-placeholder">Coolant — coming soon</div>
          }
          @case ('console') {
            <div class="tab-placeholder">Console — coming soon</div>
          }
        }
      </div>
    </section>
  `,
  styles: [`
    .job-config-tabs {
      background: var(--surface, #24283b);
      border-radius: 8px;
      border: 1px solid var(--muted, #565f89);
      overflow: hidden;
    }
    .tabs-header {
      display: flex;
      align-items: center;
      border-bottom: 1px solid var(--muted, #565f89);
      background: var(--bg, #1a1b26);
    }
    .tab-scroll {
      flex-shrink: 0;
      width: 28px;
      height: 36px;
      border: none;
      background: transparent;
      color: var(--muted, #565f89);
      cursor: pointer;
      font-size: 0.9rem;
    }
    .tab-scroll:hover { color: var(--text, #c0caf5); }
    .tabs-nav {
      display: flex;
      flex: 1;
      min-width: 0;
      gap: 0;
    }
    .tab {
      padding: 0.6rem 1rem;
      border: none;
      background: transparent;
      color: var(--muted, #565f89);
      font-size: 0.85rem;
      font-weight: 500;
      cursor: pointer;
      border-bottom: 2px solid transparent;
      white-space: nowrap;
    }
    .tab:hover { color: var(--text, #c0caf5); }
    .tab-active {
      color: var(--accent, #7aa2f7);
      border-bottom-color: var(--accent, #7aa2f7);
    }
    .tabs-content {
      padding: 0 1rem 1rem;
    }
    .tab-placeholder {
      padding: 2rem 1rem;
      text-align: center;
      color: var(--muted, #565f89);
      font-size: 0.9rem;
    }
  `],
})
export class JobConfigTabsComponent {
  readonly tabs = TABS;
  readonly activeTab = signal<JobConfigTabId>('probe');
}
