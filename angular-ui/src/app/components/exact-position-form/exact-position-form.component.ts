import { Component, signal, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

export interface ExactPosition {
  x: number;
  y: number;
  z: number;
}

@Component({
  selector: 'app-exact-position-form',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="exact-position-form" aria-label="Go to exact position">
      <div class="form-header">Go</div>
      <div class="form-row">
        <label class="axis-label">X</label>
        <input
          type="number"
          class="pos-input"
          [ngModel]="x()"
          (ngModelChange)="x.set(parseNum($event))"
          step="0.01"
          aria-label="X position"
        />
      </div>
      <div class="form-row">
        <label class="axis-label">Y</label>
        <input
          type="number"
          class="pos-input"
          [ngModel]="y()"
          (ngModelChange)="y.set(parseNum($event))"
          step="0.01"
          aria-label="Y position"
        />
      </div>
      <div class="form-row">
        <label class="axis-label">Z</label>
        <input
          type="number"
          class="pos-input"
          [ngModel]="z()"
          (ngModelChange)="z.set(parseNum($event))"
          step="0.01"
          aria-label="Z position"
        />
      </div>
      <button type="button" class="btn-go" (click)="onGo()">
        Go
      </button>
    </div>
  `,
  styles: [`
    .exact-position-form {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }
    .form-header {
      font-size: 0.75rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.04em;
      color: var(--muted, #565f89);
    }
    .form-row {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }
    .axis-label {
      font-size: 0.9rem;
      font-weight: 500;
      min-width: 1.5rem;
      color: var(--text, #c0caf5);
    }
    .pos-input {
      flex: 1;
      padding: 0.35rem 0.5rem;
      border-radius: 6px;
      border: 1px solid var(--muted, #565f89);
      background: var(--bg, #1a1b26);
      color: var(--text, #c0caf5);
      font-size: 0.9rem;
      font-family: ui-monospace, monospace;
    }
    .btn-go {
      margin-top: 0.25rem;
      padding: 0.5rem 1rem;
      border-radius: 6px;
      border: none;
      font-size: 0.9rem;
      font-weight: 600;
      cursor: pointer;
      background: var(--accent, #7aa2f7);
      color: var(--bg, #1a1b26);
    }
    .btn-go:hover { filter: brightness(1.1); }
  `],
})
export class ExactPositionFormComponent {
  readonly x = signal(0);
  readonly y = signal(0);
  readonly z = signal(0);

  readonly goToPosition = output<ExactPosition>();

  parseNum(val: string): number {
    return parseFloat(val) || 0;
  }

  onGo(): void {
    this.goToPosition.emit({
      x: this.x(),
      y: this.y(),
      z: this.z(),
    });
  }
}
