import { invoke } from "@tauri-apps/api/core";

class UIStore {
  sidebarOpen = $state(true);
  activeTab = $state<'dashboard' | 'jobs' | 'org' | 'reports' | 'settings'>('dashboard');
  openNewJobWizard = $state(false);

  notifications = $state<{id: string, message: string, type: 'info' | 'success' | 'error'}[]>([]);

  notify(message: string, type: 'info' | 'success' | 'error' = 'info') {
    const id = Math.random().toString(36).substring(7);
    this.notifications.push({ id, message, type });

    setTimeout(() => {
      this.notifications = this.notifications.filter(n => n.id !== id);
    }, 5000);
  }

  async loadPersistedState() {
    try {
      const config = await invoke<{ key: string; value: string }[]>("get_app_config");
      const sidebarEntry = config.find(c => c.key === "sidebar_collapsed");
      if (sidebarEntry) {
        this.sidebarOpen = sidebarEntry.value !== "true";
      }
    } catch {
      // On first launch or DB not ready, use defaults
    }
  }
}

export const uiStore = new UIStore();
