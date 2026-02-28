import { Component, input, output, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

export type SliderColor = 'blue' | 'red' | 'default';

@Component({
  selector: 'app-slider',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="slider-block" [class]="colorClass()">
      <div class="slider-header">
        <span class="slider-label">{{ label() }}</span>
        <span class="slider-value">{{ value() }} {{ unit() }}</span>
        <span class="slider-pct">{{ pct() }}%</span>
      </div>
      <div class="slider-row">
        <button
          type="button"
          class="btn-icon btn-reset"
          (click)="onReset()"
          aria-label="Reset"
          title="Reset"
        >
          &#8630;
        </button>
        <input
          type="range"
          class="slider-input"
          [min]="min()"
          [max]="max()"
          [ngModel]="value()"
          (ngModelChange)="emitValue(parseNum($event))"
          [attr.aria-label]="label()"
        />
        <button type="button" class="btn-icon btn-minus" (click)="onStep(-1)" aria-label="Decrease">−</button>
        <button type="button" class="btn-icon btn-plus" (click)="onStep(1)" aria-label="Increase">+</button>
      </div>
    </div>
  `,
  styles: [`
    .slider-block {
      display: flex;
      flex-direction: column;
      gap: 0.35rem;
    }
    .slider-header {
      display: flex;
      align-items: baseline;
      gap: 0.5rem;
      flex-wrap: wrap;
    }
    .slider-label {
      font-size: 0.85rem;
      font-weight: 500;
      color: var(--text, #c0caf5);
    }
    .slider-value {
      font-size: 0.9rem;
      font-family: ui-monospace, monospace;
      color: var(--text, #c0caf5);
    }
    .slider-pct {
      margin-left: auto;
      font-size: 0.8rem;
      color: var(--muted, #565f89);
    }
    .slider-row {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }
    .btn-icon {
      width: 28px;
      height: 28px;
      border-radius: 50%;
      border: 1px solid var(--muted, #565f89);
      background: var(--surface, #24283b);
      color: var(--text, #c0caf5);
      font-size: 1rem;
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
    }
    .btn-icon:hover { background: var(--muted, #565f89); }
    .btn-reset { font-size: 1.1rem; }
    .slider-input {
      flex: 1;
      min-width: 0;
      height: 8px;
      -webkit-appearance: none;
      appearance: none;
      background: var(--muted, #565f89);
      border-radius: 4px;
    }
    .slider-input::-webkit-slider-thumb {
      -webkit-appearance: none;
      width: 18px;
      height: 18px;
      border-radius: 50%;
      background: var(--text, #c0caf5);
      border: 1px solid var(--muted, #565f89);
      cursor: pointer;
    }
    .slider-input::-moz-range-thumb {
      width: 18px;
      height: 18px;
      border-radius: 50%;
      background: var(--text, #c0caf5);
      border: 1px solid var(--muted, #565f89);
      cursor: pointer;
    }
    .slider-block.slider-color-blue .slider-input::-webkit-slider-runnable-track { background: linear-gradient(to right, #7aa2f7 0%, #7aa2f7 var(--fill-pct, 0%), var(--muted, #565f89) var(--fill-pct, 0%)); }
    .slider-block.slider-color-blue .slider-input::-moz-range-track { background: linear-gradient(to right, #7aa2f7 0%, #7aa2f7 var(--fill-pct, 0%), var(--muted, #565f89) var(--fill-pct, 0%)); }
    .slider-block.slider-color-red .slider-input::-webkit-slider-runnable-track { background: linear-gradient(to right, #f7768e 0%, #f7768e var(--fill-pct, 0%), var(--muted, #565f89) var(--fill-pct, 0%)); }
    .slider-block.slider-color-red .slider-input::-moz-range-track { background: linear-gradient(to right, #f7768e 0%, #f7768e var(--fill-pct, 0%), var(--muted, #565f89) var(--fill-pct, 0%)); }
  `],
})
export class SliderComponent {
  readonly label = input<string>('');
  readonly unit = input<string>('');
  readonly min = input<number>(0);
  readonly max = input<number>(100);
  readonly step = input<number>(1);
  readonly color = input<SliderColor>('default');
  readonly resetValue = input<number>(0);
  readonly value = input<number>(0);

  readonly valueChange = output<number>();

  readonly pct = computed(() => {
    const min = this.min();
    const max = this.max();
    const v = this.value();
    if (max === min) return 100;
    return Math.round(((v - min) / (max - min)) * 100);
  });

  readonly colorClass = computed(() => {
    const c = this.color();
    if (c === 'blue') return 'slider-color-blue';
    if (c === 'red') return 'slider-color-red';
    return '';
  });

  parseNum(val: string): number {
    return parseFloat(val) || 0;
  }

  emitValue(v: number): void {
    const min = this.min();
    const max = this.max();
    const s = this.step();
    const clamped = Math.min(max, Math.max(min, v));
    const stepped = Math.round(clamped / s) * s;
    this.valueChange.emit(stepped);
  }

  onReset(): void {
    this.valueChange.emit(this.resetValue());
  }

  onStep(delta: number): void {
    const s = this.step();
    const min = this.min();
    const max = this.max();
    const next = Math.min(max, Math.max(min, this.value() + delta * s));
    this.valueChange.emit(Math.round(next / s) * s);
  }
}
