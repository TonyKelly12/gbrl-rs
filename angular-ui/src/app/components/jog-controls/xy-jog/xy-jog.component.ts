import { Component, output } from '@angular/core';
import { CommonModule } from '@angular/common';

export type XyJogDirection = 'x+' | 'x-' | 'y+' | 'y-' | 'stop';

@Component({
  selector: 'app-xy-jog',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="xy-jog" aria-label="XY jog control">
      <div class="jog-pad">
        <button
          type="button"
          class="jog-btn jog-y-plus"
          (click)="jog.emit('y+')"
          (mousedown)="jog.emit('y+')"
          (mouseup)="jog.emit('stop')"
          (mouseleave)="jog.emit('stop')"
          aria-label="Jog Y+"
        >
          Y+
        </button>
        <button
          type="button"
          class="jog-btn jog-x-minus"
          (click)="jog.emit('x-')"
          (mousedown)="jog.emit('x-')"
          (mouseup)="jog.emit('stop')"
          (mouseleave)="jog.emit('stop')"
          aria-label="Jog X-"
        >
          X-
        </button>
        <button
          type="button"
          class="jog-btn jog-stop"
          (click)="jog.emit('stop')"
          aria-label="Stop"
        >
          STOP
        </button>
        <button
          type="button"
          class="jog-btn jog-x-plus"
          (click)="jog.emit('x+')"
          (mousedown)="jog.emit('x+')"
          (mouseup)="jog.emit('stop')"
          (mouseleave)="jog.emit('stop')"
          aria-label="Jog X+"
        >
          X+
        </button>
        <button
          type="button"
          class="jog-btn jog-y-minus"
          (click)="jog.emit('y-')"
          (mousedown)="jog.emit('y-')"
          (mouseup)="jog.emit('stop')"
          (mouseleave)="jog.emit('stop')"
          aria-label="Jog Y-"
        >
          Y-
        </button>
      </div>
    </div>
  `,
  styles: [`
    .xy-jog {
      display: flex;
      justify-content: center;
      align-items: center;
    }
    .jog-pad {
      position: relative;
      width: 160px;
      height: 160px;
    }
    .jog-btn {
      position: absolute;
      border: none;
      border-radius: 8px;
      font-size: 0.75rem;
      font-weight: 600;
      cursor: pointer;
      background: var(--surface, #24283b);
      color: var(--text, #c0caf5);
      border: 1px solid var(--muted, #565f89);
      padding: 0.25rem 0.5rem;
      min-width: 44px;
    }
    .jog-btn:hover {
      background: var(--muted, #565f89);
    }
    .jog-btn:active {
      background: var(--accent, #7aa2f7);
      color: var(--bg, #1a1b26);
    }
    .jog-y-plus { top: 0;   left: 50%; transform: translateX(-50%); }
    .jog-x-minus { left: 0;  top: 50%;  transform: translateY(-50%); }
    .jog-stop {
      left: 50%;
      top: 50%;
      transform: translate(-50%, -50%);
      width: 52px;
      height: 52px;
      border-radius: 50%;
      font-size: 0.7rem;
      background: #f7768e;
      color: var(--bg, #1a1b26);
      border-color: #f7768e;
    }
    .jog-stop:hover { filter: brightness(1.1); }
    .jog-x-plus { right: 0; top: 50%;  transform: translateY(-50%); }
    .jog-y-minus { bottom: 0; left: 50%; transform: translateX(-50%); }
  `],
})
export class XyJogComponent {
  readonly jog = output<XyJogDirection>();
}
