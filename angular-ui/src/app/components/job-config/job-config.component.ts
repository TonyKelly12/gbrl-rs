import { Component, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { JobCommandComponent, type JobCommandAction } from '../job-command/job-command.component';
import { SliderComponent } from '../slider/slider.component';

@Component({
  selector: 'app-job-config',
  standalone: true,
  imports: [CommonModule, JobCommandComponent, SliderComponent],
  template: `
    <section class="job-config" aria-label="Job configuration">
      <app-job-command (action)="onJobCommand($event)"></app-job-command>
      <div class="job-config-sliders">
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
    .job-config {
      background: var(--surface, #24283b);
      border-radius: 8px;
      padding: 1rem;
      border: 1px solid var(--muted, #565f89);
    }
    .job-config-sliders {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      margin-top: 1rem;
    }
  `],
})
export class JobConfigComponent {
  readonly feedValue = signal(0);
  readonly spindleValue = signal(0);

  onJobCommand(_action: JobCommandAction): void {
    // Parent or service can handle; UI only for now
  }
}
