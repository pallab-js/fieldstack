class UIStore {
  sidebarOpen = $state(true);
  activeTab = $state<'dashboard' | 'jobs' | 'org' | 'reports' | 'settings'>('dashboard');
  
  notifications = $state<{id: string, message: string, type: 'info' | 'success' | 'error'}[]>([]);

  notify(message: string, type: 'info' | 'success' | 'error' = 'info') {
    const id = Math.random().toString(36).substring(7);
    this.notifications.push({ id, message, type });
    
    setTimeout(() => {
      this.notifications = this.notifications.filter(n => n.id !== id);
    }, 5000);
  }
}

export const uiStore = new UIStore();
