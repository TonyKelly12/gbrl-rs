import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { XyJogComponent, type XyJogDirection } from './xy-jog/xy-jog.component';
import { ZJogComponent, type ZJogDirection } from './z-jog/z-jog.component';
import { JogSpeedControlsComponent } from '../jog-speed-controls/jog-speed-controls.component';
import { ExactPositionFormComponent, type ExactPosition } from '../exact-position-form/exact-position-form.component';

@Component({
  selector: 'app-jog-controls',
  standalone: true,
  imports: [
    CommonModule,
    XyJogComponent,
    ZJogComponent,
    JogSpeedControlsComponent,
    ExactPositionFormComponent,
  ],
  template: `
    <section class="jog-controls" aria-label="Jog controls">
      <h2 class="section-title">Jog</h2>
      <div class="jog-layout">
        <div class="jog-pad-row">
          <app-xy-jog (jog)="onXyJog($event)"></app-xy-jog>
          <app-z-jog (jog)="onZJog($event)"></app-z-jog>
        </div>
        <app-jog-speed-controls (presetChange)="onPresetChange($event)"></app-jog-speed-controls>
        <app-exact-position-form (goToPosition)="onGoToPosition($event)"></app-exact-position-form>
      </div>
    </section>
  `,
  styles: [`
    .jog-controls {
      background: var(--surface, #24283b);
      border-radius: 8px;
      padding: 1rem;
      border: 1px solid var(--muted, #565f89);
    }
    .section-title {
      font-size: 0.85rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: var(--muted, #565f89);
      margin: 0 0 1rem 0;
    }
    .jog-layout {
      display: flex;
      flex-direction: column;
      gap: 1rem;
    }
    .jog-pad-row {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 1rem;
    }
  `],
})
export class JogControlsComponent {
  onXyJog(dir: XyJogDirection): void {
    // Parent or service can subscribe; UI only for now
  }

  onZJog(dir: ZJogDirection): void {
    // Parent or service can subscribe; UI only for now
  }

  onPresetChange(_preset: 'rapid' | 'normal' | 'precise'): void {
    // Optional: sync with other components
  }

  onGoToPosition(_pos: ExactPosition): void {
    // Parent or service can handle exact position command; UI only for now
  }
}
