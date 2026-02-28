import { Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';

export type MainMenuTabId = 'helper' | 'carve' | 'stats' | 'tools' | 'config';

interface MenuItem {
  id: MainMenuTabId;
  label: string;
  icon: string;
}

const MENU_ITEMS: MenuItem[] = [
  { id: 'helper', label: 'Helper', icon: '\u{1F4AC}' },
  { id: 'carve', label: 'Carve', icon: '\u{26F0}' },
  { id: 'stats', label: 'Stats', icon: '\u{1F4CA}' },
  { id: 'tools', label: 'Tools', icon: '\u{1F527}' },
  { id: 'config', label: 'Config', icon: '\u{2699}' },
];

@Component({
  selector: 'app-main-menu',
  standalone: true,
  imports: [CommonModule],
  template: `
    <nav class="main-menu" aria-label="Main navigation">
      <ul class="menu-list" role="tablist">
        @for (item of menuItems; track item.id) {
          <li>
            <button
              type="button"
              class="menu-tab"
              [class.menu-tab-active]="activeTab() === item.id"
              (click)="tabChange.emit(item.id)"
              [attr.aria-selected]="activeTab() === item.id"
              [attr.aria-label]="item.label"
              role="tab"
            >
              <span class="menu-icon" [attr.aria-hidden]="true">{{ item.icon }}</span>
              <span class="menu-label">{{ item.label }}</span>
            </button>
          </li>
        }
      </ul>
    </nav>
  `,
  styles: [`
    :host {
      display: block;
      height: 100%;
    }
    .main-menu {
      width: 72px;
      height: 100%;
      flex-shrink: 0;
      background: var(--surface, #24283b);
      border-right: 1px solid var(--muted, #565f89);
      padding: 0.5rem 0;
    }
    .menu-list {
      list-style: none;
      margin: 0;
      padding: 0;
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
    }
    .menu-tab {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 0.25rem;
      width: 100%;
      padding: 0.6rem 0.35rem;
      border: none;
      background: transparent;
      color: var(--muted, #565f89);
      font-size: 0.7rem;
      cursor: pointer;
      border-radius: 6px;
    }
    .menu-tab:hover {
      color: var(--text, #c0caf5);
      background: rgba(255, 255, 255, 0.05);
    }
    .menu-tab-active {
      color: var(--accent, #7aa2f7);
      background: rgba(122, 162, 247, 0.15);
    }
    .menu-tab-active::before {
      content: '';
      position: absolute;
      left: 0;
      top: 50%;
      transform: translateY(-50%);
      width: 3px;
      height: 60%;
      background: var(--accent, #7aa2f7);
      border-radius: 0 2px 2px 0;
    }
    .menu-tab {
      position: relative;
    }
    .menu-icon {
      font-size: 1.4rem;
      line-height: 1;
    }
    .menu-label {
      font-weight: 500;
    }
  `],
})
export class MainMenuComponent {
  readonly menuItems = MENU_ITEMS;
  readonly activeTab = input.required<MainMenuTabId>();

  readonly tabChange = output<MainMenuTabId>();
}
