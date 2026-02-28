import { Component, output } from '@angular/core';
import { CommonModule } from '@angular/common';

export type ZJogDirection = 'z+' | 'z-' | 'stop';

@Component({
  selector: 'app-z-jog',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="z-jog" aria-label="Z jog control">
      <button
        type="button"
        class="z-btn z-plus"
        (click)="jog.emit('z+')"
        (mousedown)="jog.emit('z+')"
        (mouseup)="jog.emit('stop')"
        (mouseleave)="jog.emit('stop')"
        aria-label="Jog Z+"
      >
        Z+
      </button>
      <button
        type="button"
        class="z-btn z-minus"
        (click)="jog.emit('z-')"
        (mousedown)="jog.emit('z-')"
        (mouseup)="jog.emit('stop')"
        (mouseleave)="jog.emit('stop')"
        aria-label="Jog Z-"
      >
        Z-
      </button>
    </div>
  `,
  styles: [`
    .z-jog {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      align-items: center;
    }
    .z-btn {
      padding: 0.5rem 1rem;
      min-width: 56px;
      border: none;
      border-radius: 6px;
      font-size: 0.85rem;
      font-weight: 600;
      cursor: pointer;
      background: var(--surface, #24283b);
      color: var(--text, #c0caf5);
      border: 1px solid var(--muted, #565f89);
    }
    .z-btn:hover {
      background: var(--muted, #565f89);
    }
    .z-btn:active {
      background: var(--accent, #7aa2f7);
      color: var(--bg, #1a1b26);
    }
  `],
})
export class ZJogComponent {
  readonly jog = output<ZJogDirection>();
}
