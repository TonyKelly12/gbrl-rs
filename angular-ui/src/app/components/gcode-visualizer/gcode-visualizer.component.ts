import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { WebglPreviewComponent } from '../webgl-preview/webgl-preview.component';

@Component({
  selector: 'app-gcode-visualizer',
  standalone: true,
  imports: [CommonModule, WebglPreviewComponent],
  template: `
    <section class="gcode-visualizer" aria-label="G-code 3D preview">
      <app-webgl-preview></app-webgl-preview>
    </section>
  `,
  styles: [`
    :host {
      display: block;
      width: 100%;
      height: 100%;
      min-height: 320px;
    }
    .gcode-visualizer {
      width: 100%;
      height: 100%;
      min-height: 320px;
      background: var(--surface, #24283b);
      border-radius: 8px;
      border: 1px solid var(--muted, #565f89);
      overflow: hidden;
    }
  `],
})
export class GcodeVisualizerComponent {}
