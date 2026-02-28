import {
  Component,
  OnDestroy,
  AfterViewInit,
  ViewChild,
  ElementRef,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

@Component({
  selector: 'app-webgl-preview',
  standalone: true,
  imports: [CommonModule],
  template: `<div #container class="webgl-container"></div>`,
  styles: [`
    :host {
      display: block;
      width: 100%;
      height: 100%;
      min-height: 320px;
    }
    .webgl-container {
      width: 100%;
      height: 100%;
      min-height: 320px;
      position: relative;
    }
    .webgl-container canvas {
      display: block;
      width: 100%;
      height: 100%;
    }
  `],
})
export class WebglPreviewComponent implements AfterViewInit, OnDestroy {
  @ViewChild('container', { static: true }) containerRef!: ElementRef<HTMLDivElement>;

  private scene: THREE.Scene | null = null;
  private camera: THREE.PerspectiveCamera | null = null;
  private renderer: THREE.WebGLRenderer | null = null;
  private controls: OrbitControls | null = null;
  private frameId: number | null = null;
  private resizeObserver: ResizeObserver | null = null;

  ngAfterViewInit(): void {
    const container = this.containerRef.nativeElement;
    if (!container) return;

    const width = container.clientWidth;
    const height = container.clientHeight;

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1a1b26);

    const camera = new THREE.PerspectiveCamera(50, width / height, 0.1, 1000);
    camera.position.set(80, 60, 80);
    camera.lookAt(0, 0, 0);

    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;
    controls.target.set(0, 0, 0);

    const gridSize = 150;
    const gridDivisions = 30;
    const gridHelper = new THREE.GridHelper(gridSize, gridDivisions, 0x3b4261, 0x24283b);
    scene.add(gridHelper);

    const axesSize = 40;
    const axesHelper = new THREE.AxesHelper(axesSize);
    scene.add(axesHelper);

    const ambient = new THREE.AmbientLight(0x404060);
    scene.add(ambient);
    const dir = new THREE.DirectionalLight(0xc0caf5, 0.8);
    dir.position.set(30, 50, 30);
    scene.add(dir);

    this.scene = scene;
    this.camera = camera;
    this.renderer = renderer;
    this.controls = controls;

    const onResize = (): void => {
      const w = container.clientWidth;
      const h = container.clientHeight;
      if (camera && renderer) {
        camera.aspect = w / h;
        camera.updateProjectionMatrix();
        renderer.setSize(w, h);
      }
    };
    this.resizeObserver = new ResizeObserver(onResize);
    this.resizeObserver.observe(container);

    const animate = (): void => {
      this.frameId = requestAnimationFrame(animate);
      controls.update();
      renderer.render(scene, camera);
    };
    animate();
  }

  ngOnDestroy(): void {
    if (this.frameId !== null) {
      cancelAnimationFrame(this.frameId);
    }
    this.resizeObserver?.disconnect();
    this.resizeObserver = null;
    this.controls?.dispose();
    this.renderer?.dispose();
    if (this.renderer?.domElement?.parentNode) {
      this.renderer.domElement.parentNode.removeChild(this.renderer.domElement);
    }
    this.scene = null;
    this.camera = null;
    this.renderer = null;
    this.controls = null;
  }
}
