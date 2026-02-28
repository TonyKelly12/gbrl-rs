import { Component, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { JobCommandComponent, type JobCommandAction } from '../job-command/job-command.component';
import { SliderComponent } from '../slider/slider.component';

@Component({
  selector: 'app-run-job-panel',
  standalone: true,
  imports: [CommonModule, JobCommandComponent, SliderComponent],
  template: `
    <section class="run-job-panel" aria-label="Run job">
      <app-job-command (action)="onJobCommand($event)"></app-job-command>
      <div class="run-job-sliders">
        <app-slider
          label="Feed"
          unit="mm/min"
          [min]="0"
          [max]="5000"
          [step]="100"
          [value]="feedValue()"
          [resetValue]="0"
          color="blue"
          (valueChange)="feedValue.set($event)"
        />
        <app-slider
          label="Spindle"
          unit="RPM"
          [min]="0"
          [max]="24000"
          [step]="100"
          [value]="spindleValue()"
          [resetValue]="0"
          color="red"
          (valueChange)="spindleValue.set($event)"
        />
      </div>
    </section>
  `,
  styles: [`
    .run-job-panel {
      background: var(--surface, #24283b);
      border-radius: 8px;
      padding: 1rem;
      border: 1px solid var(--muted, #565f89);
    }
    .run-job-sliders {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      margin-top: 1rem;
    }
  `],
})
export class RunJobPanelComponent {
  readonly feedValue = signal(0);
  readonly spindleValue = signal(0);

  onJobCommand(_action: JobCommandAction): void {
    // Parent or service can handle; UI only for now
  }
}
