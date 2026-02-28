import { Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';

export interface CommandButtonOption {
  id: string;
  label: string;
}

@Component({
  selector: 'app-command-button',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="command-button-group" aria-label="Command buttons">
      @for (btn of buttons(); track btn.id) {
        <button
          type="button"
          class="cmd-btn"
          [class.cmd-btn-active]="selectedId() === btn.id"
          (click)="buttonClick.emit(btn.id)"
          [attr.aria-pressed]="selectedId() === btn.id"
          [attr.aria-label]="btn.label"
        >
          {{ btn.label }}
        </button>
      }
    </div>
  `,
  styles: [`
    .command-button-group {
      display: flex;
      flex-wrap: wrap;
      gap: 0.35rem;
    }
    .cmd-btn {
      padding: 0.45rem 0.75rem;
      border-radius: 6px;
      border: 1px solid var(--muted, #565f89);
      background: var(--surface, #24283b);
      color: var(--text, #c0caf5);
      font-size: 0.85rem;
      font-weight: 500;
      cursor: pointer;
    }
    .cmd-btn:hover {
      background: var(--muted, #565f89);
    }
    .cmd-btn-active {
      background: var(--accent, #7aa2f7);
      color: var(--bg, #1a1b26);
      border-color: var(--accent, #7aa2f7);
    }
  `],
})
export class CommandButtonComponent {
  readonly buttons = input<CommandButtonOption[]>([]);
  readonly selectedId = input<string | undefined>(undefined);

  readonly buttonClick = output<string>();
}
