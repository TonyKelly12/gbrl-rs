import { Component, signal, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

export type JogSpeedPreset = 'rapid' | 'normal' | 'precise';

@Component({
  selector: 'app-jog-speed-controls',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="jog-speed-controls" aria-label="Jog speed and step">
      <div class="row">
        <label class="label">XY:</label>
        <input
          type="number"
          class="input-num"
          [ngModel]="xyStep()"
          (ngModelChange)="xyStep.set(parseNum($event))"
          min="0.01"
          step="0.1"
          aria-label="XY step size"
        />
      </div>
      <div class="row">
        <label class="label">Z:</label>
        <input
          type="number"
          class="input-num"
          [ngModel]="zStep()"
          (ngModelChange)="zStep.set(parseNum($event))"
          min="0.01"
          step="0.1"
          aria-label="Z step size"
        />
      </div>
      <div class="row">
        <label class="label">at:</label>
        <input
          type="number"
          class="input-num"
          [ngModel]="feedRate()"
          (ngModelChange)="feedRate.set(parseNum($event))"
          min="1"
          step="100"
          aria-label="Jog feed rate"
        />
      </div>
      <div class="presets">
        <button
          type="button"
          class="preset-btn"
          [class.active]="preset() === 'rapid'"
          (click)="setPreset('rapid')"
        >
          Rapid
        </button>
        <button
          type="button"
          class="preset-btn"
          [class.active]="preset() === 'normal'"
          (click)="setPreset('normal')"
        >
          Normal
        </button>
        <button
          type="button"
          class="preset-btn"
          [class.active]="preset() === 'precise'"
          (click)="setPreset('precise')"
        >
          Precise
        </button>
      </div>
    </div>
  `,
  styles: [`
    .jog-speed-controls {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }
    .row {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }
    .label {
      font-size: 0.8rem;
      color: var(--muted, #565f89);
      min-width: 2ch;
    }
    .input-num {
      width: 4rem;
      padding: 0.35rem 0.5rem;
      border-radius: 6px;
      border: 1px solid var(--muted, #565f89);
      background: var(--bg, #1a1b26);
      color: var(--text, #c0caf5);
      font-size: 0.9rem;
      font-family: ui-monospace, monospace;
    }
    .presets {
      display: flex;
      gap: 0.35rem;
      margin-top: 0.25rem;
    }
    .preset-btn {
      flex: 1;
      padding: 0.35rem 0.5rem;
      border-radius: 6px;
      border: 1px solid var(--muted, #565f89);
      background: var(--surface, #24283b);
      color: var(--text, #c0caf5);
      font-size: 0.8rem;
      cursor: pointer;
    }
    .preset-btn:hover { background: var(--muted, #565f89); }
    .preset-btn.active {
      background: var(--accent, #7aa2f7);
      color: var(--bg, #1a1b26);
      border-color: var(--accent, #7aa2f7);
    }
  `],
})
export class JogSpeedControlsComponent {
  readonly xyStep = signal(5);
  readonly zStep = signal(2);
  readonly feedRate = signal(3000);
  readonly preset = signal<JogSpeedPreset>('normal');

  readonly presetChange = output<JogSpeedPreset>();

  parseNum(val: string): number {
    return parseFloat(val) || 0;
  }

  setPreset(p: JogSpeedPreset): void {
    this.preset.set(p);
    this.presetChange.emit(p);
    if (p === 'rapid') {
      this.xyStep.set(10);
      this.zStep.set(5);
      this.feedRate.set(5000);
    } else if (p === 'normal') {
      this.xyStep.set(5);
      this.zStep.set(2);
      this.feedRate.set(3000);
    } else {
      this.xyStep.set(0.1);
      this.zStep.set(0.1);
      this.feedRate.set(500);
    }
  }
}
