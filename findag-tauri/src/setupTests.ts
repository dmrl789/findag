import '@testing-library/jest-dom';

// Mock Tauri APIs
Object.defineProperty(window, '__TAURI__', {
  value: {
    invoke: jest.fn(),
    event: {
      listen: jest.fn(),
      emit: jest.fn(),
    },
    window: {
      getCurrent: jest.fn(() => ({
        close: jest.fn(),
        minimize: jest.fn(),
        maximize: jest.fn(),
        unmaximize: jest.fn(),
        setFullscreen: jest.fn(),
        setSize: jest.fn(),
        setPosition: jest.fn(),
        setAlwaysOnTop: jest.fn(),
        setContentProtected: jest.fn(),
        setTitle: jest.fn(),
        setIcon: jest.fn(),
        setSkipTaskbar: jest.fn(),
        setCursorGrab: jest.fn(),
        setCursorVisible: jest.fn(),
        setCursorIcon: jest.fn(),
        setCursorPosition: jest.fn(),
        setCursorBlink: jest.fn(),
        setFocus: jest.fn(),
        setIgnoreCursorEvents: jest.fn(),
        setDecorations: jest.fn(),
        setResizable: jest.fn(),
        setMinSize: jest.fn(),
        setMaxSize: jest.fn(),
        setVisible: jest.fn(),
        setCenter: jest.fn(),
        requestUserAttention: jest.fn(),
        setProgressBar: jest.fn(),
        setMenu: jest.fn(),
        setTheme: jest.fn(),
        setParent: jest.fn(),
        setChild: jest.fn(),
        setAutoResize: jest.fn(),
        setAlwaysOnTop: jest.fn(),
        setContentProtected: jest.fn(),
        setTitle: jest.fn(),
        setIcon: jest.fn(),
        setSkipTaskbar: jest.fn(),
        setCursorGrab: jest.fn(),
        setCursorVisible: jest.fn(),
        setCursorIcon: jest.fn(),
        setCursorPosition: jest.fn(),
        setCursorBlink: jest.fn(),
        setFocus: jest.fn(),
        setIgnoreCursorEvents: jest.fn(),
        setDecorations: jest.fn(),
        setResizable: jest.fn(),
        setMinSize: jest.fn(),
        setMaxSize: jest.fn(),
        setVisible: jest.fn(),
        setCenter: jest.fn(),
        requestUserAttention: jest.fn(),
        setProgressBar: jest.fn(),
        setMenu: jest.fn(),
        setTheme: jest.fn(),
        setParent: jest.fn(),
        setChild: jest.fn(),
        setAutoResize: jest.fn(),
      })),
    },
    app: {
      getVersion: jest.fn(() => Promise.resolve('1.0.0')),
      getName: jest.fn(() => Promise.resolve('FinDAG')),
      getTauriVersion: jest.fn(() => Promise.resolve('1.0.0')),
    },
    fs: {
      readTextFile: jest.fn(),
      writeTextFile: jest.fn(),
      readBinaryFile: jest.fn(),
      writeBinaryFile: jest.fn(),
      readDir: jest.fn(),
      createDir: jest.fn(),
      removeDir: jest.fn(),
      removeFile: jest.fn(),
      renameFile: jest.fn(),
      exists: jest.fn(),
    },
    path: {
      join: jest.fn(),
      dirname: jest.fn(),
      extname: jest.fn(),
      basename: jest.fn(),
      isAbsolute: jest.fn(),
    },
    os: {
      platform: jest.fn(() => Promise.resolve('win32')),
      type: jest.fn(() => Promise.resolve('Windows_NT')),
      version: jest.fn(() => Promise.resolve('10.0.19044')),
      arch: jest.fn(() => Promise.resolve('x86_64')),
      tempdir: jest.fn(() => Promise.resolve('/tmp')),
      homedir: jest.fn(() => Promise.resolve('/home/user')),
    },
    shell: {
      open: jest.fn(),
      command: jest.fn(),
    },
    dialog: {
      open: jest.fn(),
      save: jest.fn(),
      message: jest.fn(),
      ask: jest.fn(),
      confirm: jest.fn(),
    },
    notification: {
      isPermissionGranted: jest.fn(() => Promise.resolve(true)),
      requestPermission: jest.fn(() => Promise.resolve('granted')),
      sendNotification: jest.fn(),
    },
    globalShortcut: {
      register: jest.fn(),
      unregister: jest.fn(),
      unregisterAll: jest.fn(),
      isRegistered: jest.fn(),
    },
    clipboard: {
      writeText: jest.fn(),
      readText: jest.fn(),
    },
    http: {
      fetch: jest.fn(),
      getClient: jest.fn(),
    },
    store: {
      get: jest.fn(),
      set: jest.fn(),
      delete: jest.fn(),
      clear: jest.fn(),
      reset: jest.fn(),
      save: jest.fn(),
      load: jest.fn(),
      onKeyChange: jest.fn(),
    },
  },
  writable: true,
});

// Mock localStorage
const localStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
Object.defineProperty(window, 'localStorage', {
  value: localStorageMock,
});

// Mock sessionStorage
const sessionStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
Object.defineProperty(window, 'sessionStorage', {
  value: sessionStorageMock,
});

// Mock ResizeObserver
global.ResizeObserver = jest.fn().mockImplementation(() => ({
  observe: jest.fn(),
  unobserve: jest.fn(),
  disconnect: jest.fn(),
}));

// Mock IntersectionObserver
global.IntersectionObserver = jest.fn().mockImplementation(() => ({
  observe: jest.fn(),
  unobserve: jest.fn(),
  disconnect: jest.fn(),
}));

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: jest.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: jest.fn(), // deprecated
    removeListener: jest.fn(), // deprecated
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
});

// Mock requestAnimationFrame
global.requestAnimationFrame = jest.fn(callback => setTimeout(callback, 0));
global.cancelAnimationFrame = jest.fn(id => clearTimeout(id));

// Mock console methods to reduce noise in tests
const originalConsoleError = console.error;
const originalConsoleWarn = console.warn;

beforeAll(() => {
  console.error = (...args: any[]) => {
    if (
      typeof args[0] === 'string' &&
      args[0].includes('Warning: ReactDOM.render is no longer supported')
    ) {
      return;
    }
    originalConsoleError.call(console, ...args);
  };

  console.warn = (...args: any[]) => {
    if (
      typeof args[0] === 'string' &&
      (args[0].includes('Warning: componentWillReceiveProps') ||
        args[0].includes('Warning: componentWillUpdate'))
    ) {
      return;
    }
    originalConsoleWarn.call(console, ...args);
  };
});

afterAll(() => {
  console.error = originalConsoleError;
  console.warn = originalConsoleWarn;
});

// Global test utilities
global.testUtils = {
  waitForElementToBeRemoved: (element: Element) =>
    new Promise(resolve => {
      const observer = new MutationObserver(() => {
        if (!document.contains(element)) {
          observer.disconnect();
          resolve(undefined);
        }
      });
      observer.observe(document.body, { childList: true, subtree: true });
    }),
}; 