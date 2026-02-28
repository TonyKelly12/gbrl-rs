import { Component, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CommandButtonComponent } from '../command-button/command-button.component';

const PROBE_AXIS_BUTTONS = [
  { id: 'z', label: 'Z' },
  { id: 'xyz', label: 'XYZ' },
  { id: 'xy', label: 'XY' },
  { id: 'x', label: 'X' },
  { id: 'y', label: 'Y' },
];

@Component({
  selector: 'app-probe-tab',
  standalone: true,
  imports: [CommonModule, CommandButtonComponent],
  template: `
    <div class="probe-tab" aria-label="Probe">
      <div class="probe-tab-main">
        <div class="probe-controls">
          <app-command-button
            [buttons]="axisButtons"
            [selectedId]="selectedAxis()"
            (buttonClick)="onAxisClick($event)"
          ></app-command-button>
          <button type="button" class="probe-btn" (click)="onProbe()" aria-label="Run probe">
            Probe
          </button>
        </div>
        <div class="probe-illustration" [attr.data-axis]="selectedAxis()">
          <span class="illustration-placeholder">{{ illustrationLabel() }}</span>
          <button type="button" class="illustration-collapse" aria-label="Collapse illustration">&#8690;</button>
        </div>
      </div>
    </div>
  `,
  styles: [`
    .probe-tab {
      padding: 0.5rem 0;
    }
    .probe-tab-main {
      display: flex;
      gap: 1rem;
      align-items: flex-start;
    }
    .probe-controls {
      display: flex;
      flex-direction: column;
      gap: 0.75rem;
      flex-shrink: 0;
    }
    .probe-btn {
      padding: 0.5rem 1.25rem;
      border-radius: 6px;
      border: 1px solid var(--muted, #565f89);
      background: var(--surface, #24283b);
      color: var(--text, #c0caf5);
      font-size: 0.9rem;
      font-weight: 600;
      cursor: pointer;
    }
    .probe-btn:hover { background: var(--muted, #565f89); }
    .probe-illustration {
      flex: 1;
      min-width: 120px;
      min-height: 140px;
      border-radius: 8px;
      border: 1px dashed var(--muted, #565f89);
      background: var(--bg, #1a1b26);
      position: relative;
      display: flex;
      align-items: center;
      justify-content: center;
    }
    .illustration-placeholder {
      font-size: 0.85rem;
      color: var(--muted, #565f89);
    }
    .illustration-collapse {
      position: absolute;
      top: 0.35rem;
      right: 0.35rem;
      width: 24px;
      height: 24px;
      border-radius: 4px;
      border: none;
      background: var(--accent, #7aa2f7);
      color: var(--bg, #1a1b26);
      font-size: 0.9rem;
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
    }
  `],
})
export class ProbeTabComponent {
  readonly axisButtons = PROBE_AXIS_BUTTONS;
  readonly selectedAxis = signal<string>('z');

  readonly illustrationLabel = computed(() => {
    const id = this.selectedAxis();
    const labels: Record<string, string> = {
      z: 'Z probe',
      xyz: 'XYZ probe',
      xy: 'XY probe',
      x: 'X probe',
      y: 'Y probe',
    };
    return labels[id] ?? 'Probe';
  });

  onAxisClick(id: string): void {
    this.selectedAxis.set(id);
  }

  onProbe(): void {
    // Parent or service can handle; UI only
  }
}
