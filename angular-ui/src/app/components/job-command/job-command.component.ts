import { Component, output } from '@angular/core';
import { CommonModule } from '@angular/common';

export type JobCommandAction = 'outline' | 'startFrom' | 'start' | 'pause' | 'stop';

@Component({
  selector: 'app-job-command',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="job-command" aria-label="Job commands">
      <div class="job-command-row job-command-row-top">
        <button type="button" class="btn-outline" (click)="action.emit('outline')" aria-label="Outline">
          <span class="btn-icon" aria-hidden="true">&#9634;</span>
          Outline
        </button>
        <button type="button" class="btn-outline" (click)="action.emit('startFrom')" aria-label="Start from">
          <span class="btn-icon" aria-hidden="true">&#9776;</span>
          Start From
        </button>
      </div>
      <div class="job-command-row job-command-row-main">
        <button type="button" class="btn-start" (click)="action.emit('start')" aria-label="Start">
          <span class="btn-play" aria-hidden="true">&#9654;</span>
          Start
        </button>
        <button type="button" class="btn-pause" (click)="action.emit('pause')" aria-label="Pause">
          <span class="btn-pause-icon" aria-hidden="true">&#10074;&#10074;</span>
          Pause
        </button>
        <button type="button" class="btn-stop" (click)="action.emit('stop')" aria-label="Stop">
          <span class="btn-stop-icon" aria-hidden="true">&#9632;</span>
          Stop
        </button>
      </div>
    </div>
  `,
  styles: [`
    .job-command {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }
    .job-command-row {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      flex-wrap: wrap;
    }
    .job-command-row-top .btn-outline {
      flex: 1;
      min-width: 0;
    }
    .job-command-row-main {
      justify-content: center;
      gap: 0.75rem;
    }
    .job-command button {
      border: none;
      border-radius: 6px;
      padding: 0.5rem 0.75rem;
      font-size: 0.85rem;
      font-weight: 600;
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      gap: 0.35rem;
    }
    .btn-outline {
      background: var(--surface, #24283b);
      color: var(--text, #c0caf5);
      border: 1px solid var(--muted, #565f89);
    }
    .btn-outline:hover { background: var(--muted, #565f89); }
    .btn-icon { font-size: 1rem; opacity: 0.9; }
    .btn-start {
      background: #9ece6a;
      color: var(--bg, #1a1b26);
    }
    .btn-start:hover { filter: brightness(1.1); }
    .btn-play { font-size: 1.1rem; }
    .btn-pause {
      background: var(--muted, #565f89);
      color: var(--text, #c0caf5);
    }
    .btn-pause:hover { filter: brightness(1.1); }
    .btn-pause-icon { font-size: 0.9rem; letter-spacing: 0.05em; }
    .btn-stop {
      background: #f7768e;
      color: var(--bg, #1a1b26);
    }
    .btn-stop:hover { filter: brightness(1.1); }
    .btn-stop-icon { font-size: 0.85rem; }
  `],
})
export class JobCommandComponent {
  readonly action = output<JobCommandAction>();
}
