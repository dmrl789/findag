/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_ALPHA_VANTAGE_API_KEY: string
  readonly VITE_POLYGON_API_KEY: string
  readonly VITE_FINNHUB_API_KEY: string
  readonly VITE_ENABLE_REAL_DATA: string
  readonly VITE_DATA_REFRESH_INTERVAL: string
  readonly VITE_DATA_DELAY: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
} 