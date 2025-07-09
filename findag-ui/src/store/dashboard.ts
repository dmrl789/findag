import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export interface DashboardWidget {
  id: string;
  type: string;
  size: string;
  position: number;
  config?: Record<string, any>;
}

export interface DashboardLayout {
  id: string;
  name: string;
  widgets: DashboardWidget[];
  gridTemplate: string;
  createdAt: number;
  updatedAt: number;
}

export type LayoutType = 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4' | 'grid-cols-1 lg:grid-cols-2' | 'grid-cols-1 md:grid-cols-3' | 'grid-cols-1';

interface DashboardState {
  // Current state
  widgets: DashboardWidget[];
  layout: LayoutType;
  isEditMode: boolean;
  currentLayoutId: string | null;
  
  // Saved layouts
  savedLayouts: DashboardLayout[];
  
  // Actions
  toggleEditMode: () => void;
  addWidget: (type: string, size?: string) => void;
  removeWidget: (widgetId: string) => void;
  updateWidgetPosition: (widgetId1: string, widgetId2: string) => void;
  updateWidgetConfig: (widgetId: string, config: Record<string, any>) => void;
  setLayout: (layout: LayoutType) => void;
  
  // Layout management
  saveLayout: (name?: string) => void;
  loadLayout: (layoutId?: string) => void;
  resetLayout: () => void;
  deleteLayout: (layoutId: string) => void;
  getDefaultLayout: () => DashboardLayout;
}

// Default widgets for new users
const getDefaultWidgets = (): DashboardWidget[] => [
  {
    id: 'widget-1',
    type: 'network-metrics',
    size: 'col-span-1',
    position: 0,
  },
  {
    id: 'widget-2',
    type: 'active-validators',
    size: 'col-span-1',
    position: 1,
  },
  {
    id: 'widget-3',
    type: 'hash-rate',
    size: 'col-span-1',
    position: 2,
  },
  {
    id: 'widget-4',
    type: 'block-time',
    size: 'col-span-1',
    position: 3,
  },
  {
    id: 'widget-5',
    type: 'recent-blocks',
    size: 'col-span-1 lg:col-span-1',
    position: 4,
  },
  {
    id: 'widget-6',
    type: 'recent-transactions',
    size: 'col-span-1 lg:col-span-1',
    position: 5,
  },
];

const generateWidgetId = (): string => {
  return `widget-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
};

export const useDashboardStore = create<DashboardState>()(
  persist(
    (set, get) => ({
      // Initial state
      widgets: getDefaultWidgets(),
      layout: 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4',
      isEditMode: false,
      currentLayoutId: null,
      savedLayouts: [],

      // Toggle edit mode
      toggleEditMode: () => {
        set((state) => ({
          isEditMode: !state.isEditMode,
        }));
      },

      // Add a new widget
      addWidget: (type: string, size: string = 'col-span-1') => {
        const newWidget: DashboardWidget = {
          id: generateWidgetId(),
          type,
          size,
          position: get().widgets.length,
        };

        set((state) => ({
          widgets: [...state.widgets, newWidget],
        }));
      },

      // Remove a widget
      removeWidget: (widgetId: string) => {
        set((state) => ({
          widgets: state.widgets.filter((widget) => widget.id !== widgetId),
        }));
      },

      // Update widget position (swap two widgets)
      updateWidgetPosition: (widgetId1: string, widgetId2: string) => {
        set((state) => {
          const widgets = [...state.widgets];
          const index1 = widgets.findIndex((w) => w.id === widgetId1);
          const index2 = widgets.findIndex((w) => w.id === widgetId2);

          if (index1 !== -1 && index2 !== -1) {
            // Swap positions
            const temp = widgets[index1].position;
            widgets[index1].position = widgets[index2].position;
            widgets[index2].position = temp;

            // Sort by position
            widgets.sort((a, b) => a.position - b.position);
          }

          return { widgets };
        });
      },

      // Update widget configuration
      updateWidgetConfig: (widgetId: string, config: Record<string, any>) => {
        set((state) => ({
          widgets: state.widgets.map((widget) =>
            widget.id === widgetId
              ? { ...widget, config: { ...widget.config, ...config } }
              : widget
          ),
        }));
      },

      // Set layout grid template
      setLayout: (layout: LayoutType) => {
        set({ layout });
      },

      // Save current layout
      saveLayout: (name?: string) => {
        const state = get();
        const layoutName = name || `Layout ${state.savedLayouts.length + 1}`;
        
        const newLayout: DashboardLayout = {
          id: `layout-${Date.now()}`,
          name: layoutName,
          widgets: [...state.widgets],
          gridTemplate: state.layout,
          createdAt: Date.now(),
          updatedAt: Date.now(),
        };

        set((state) => ({
          savedLayouts: [...state.savedLayouts, newLayout],
          currentLayoutId: newLayout.id,
        }));
      },

      // Load a saved layout
      loadLayout: (layoutId?: string) => {
        const state = get();
        const targetLayoutId = layoutId || state.currentLayoutId;
        
        if (targetLayoutId) {
          const layout = state.savedLayouts.find((l) => l.id === targetLayoutId);
          if (layout) {
            set({
              widgets: [...layout.widgets],
              layout: layout.gridTemplate as LayoutType,
              currentLayoutId: layout.id,
            });
          }
        } else {
          // Load default layout
          const defaultLayout = get().getDefaultLayout();
          set({
            widgets: [...defaultLayout.widgets],
            layout: defaultLayout.gridTemplate as LayoutType,
            currentLayoutId: null,
          });
        }
      },

      // Reset to default layout
      resetLayout: () => {
        const defaultLayout = get().getDefaultLayout();
        set({
          widgets: [...defaultLayout.widgets],
          layout: defaultLayout.gridTemplate as LayoutType,
          currentLayoutId: null,
        });
      },

      // Delete a saved layout
      deleteLayout: (layoutId: string) => {
        set((state) => ({
          savedLayouts: state.savedLayouts.filter((layout) => layout.id !== layoutId),
          currentLayoutId: state.currentLayoutId === layoutId ? null : state.currentLayoutId,
        }));
      },

      // Get default layout
      getDefaultLayout: (): DashboardLayout => {
        return {
          id: 'default',
          name: 'Default Layout',
          widgets: getDefaultWidgets(),
          gridTemplate: 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4',
          createdAt: Date.now(),
          updatedAt: Date.now(),
        };
      },
    }),
    {
      name: 'findag-dashboard',
      partialize: (state) => ({
        widgets: state.widgets,
        layout: state.layout,
        savedLayouts: state.savedLayouts,
        currentLayoutId: state.currentLayoutId,
      }),
    }
  )
);

// Widget type definitions
export const WIDGET_TYPES = {
  'network-metrics': {
    name: 'Network Metrics',
    description: 'Total transactions and growth',
    icon: 'BarChart3',
    defaultSize: 'col-span-1',
  },
  'active-validators': {
    name: 'Active Validators',
    description: 'Validator count and status',
    icon: 'Shield',
    defaultSize: 'col-span-1',
  },
  'hash-rate': {
    name: 'Hash Rate',
    description: 'Network hash rate and growth',
    icon: 'Zap',
    defaultSize: 'col-span-1',
  },
  'block-time': {
    name: 'Block Time',
    description: 'Average block time',
    icon: 'Clock',
    defaultSize: 'col-span-1',
  },
  'recent-blocks': {
    name: 'Recent Blocks',
    description: 'Latest blockchain blocks',
    icon: 'Blocks',
    defaultSize: 'col-span-1 lg:col-span-1',
  },
  'recent-transactions': {
    name: 'Recent Transactions',
    description: 'Latest network transactions',
    icon: 'Zap',
    defaultSize: 'col-span-1 lg:col-span-1',
  },
  'current-round': {
    name: 'Current Round',
    description: 'Active consensus round',
    icon: 'Clock',
    defaultSize: 'col-span-1',
  },
} as const;

// Layout presets
export const LAYOUT_PRESETS = {
  'default': {
    name: 'Default',
    gridTemplate: 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4',
    description: 'Standard 4-column layout',
  },
  'compact': {
    name: 'Compact',
    gridTemplate: 'grid-cols-1 md:grid-cols-3',
    description: 'Compact 3-column layout',
  },
  'wide': {
    name: 'Wide',
    gridTemplate: 'grid-cols-1 lg:grid-cols-2',
    description: 'Wide 2-column layout',
  },
  'single': {
    name: 'Single Column',
    gridTemplate: 'grid-cols-1',
    description: 'Single column layout',
  },
} as const; 