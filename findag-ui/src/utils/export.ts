import { formatTimestamp, formatNumber, formatPrice } from './formatters';

export interface ExportOptions {
  format: 'csv' | 'json' | 'excel';
  filename?: string;
  includeHeaders?: boolean;
  dateFormat?: string;
  numberFormat?: 'locale' | 'raw';
  customHeaders?: Record<string, string>;
  excludeColumns?: string[];
  transformData?: (data: any[]) => any[];
}

export interface PartialExportOptions extends Partial<ExportOptions> {
  format: 'csv' | 'json' | 'excel';
}

export interface ExportColumn {
  key: string;
  header: string;
  formatter?: (value: any) => string;
  type?: 'string' | 'number' | 'date' | 'boolean';
}

export class DataExporter {
  private static readonly DEFAULT_OPTIONS: ExportOptions = {
    format: 'csv',
    includeHeaders: true,
    dateFormat: 'ISO',
    numberFormat: 'locale',
  };

  /**
   * Export data to CSV format
   */
  static exportToCSV<T>(
    data: T[],
    columns: ExportColumn[],
    options: PartialExportOptions = { format: 'csv' }
  ): string {
    const opts = { ...this.DEFAULT_OPTIONS, ...options };
    const filteredColumns = this.filterColumns(columns, opts.excludeColumns);
    
    let csv = '';

    // Add headers
    if (opts.includeHeaders) {
      const headers = filteredColumns.map(col => 
        this.escapeCSVValue(this.getHeader(col, opts.customHeaders))
      );
      csv += headers.join(',') + '\n';
    }

    // Add data rows
    const processedData = opts.transformData ? opts.transformData(data) : data;
    
    processedData.forEach((item: any) => {
      const row = filteredColumns.map(col => {
        const value = item[col.key];
        const formattedValue = this.formatValue(value, col, opts);
        return this.escapeCSVValue(formattedValue);
      });
      csv += row.join(',') + '\n';
    });

    return csv;
  }

  /**
   * Export data to JSON format
   */
  static exportToJSON<T>(
    data: T[],
    columns: ExportColumn[],
    options: PartialExportOptions = { format: 'json' }
  ): string {
    const opts = { ...this.DEFAULT_OPTIONS, ...options };
    const filteredColumns = this.filterColumns(columns, opts.excludeColumns);
    const processedData = opts.transformData ? opts.transformData(data) : data;

    const exportData = processedData.map((item: any) => {
      const exportItem: any = {};
      filteredColumns.forEach(col => {
        const value = item[col.key];
        exportItem[col.header] = this.formatValue(value, col, opts);
      });
      return exportItem;
    });

    return JSON.stringify(exportData, null, 2);
  }

  /**
   * Download exported data as a file
   */
  static downloadExport(
    content: string,
    filename: string,
    format: 'csv' | 'json' | 'excel'
  ): void {
    const mimeTypes = {
      csv: 'text/csv;charset=utf-8;',
      json: 'application/json;charset=utf-8;',
      excel: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet;charset=utf-8;',
    };

    const blob = new Blob([content], { type: mimeTypes[format] });
    const link = document.createElement('a');
    
    if (link.download !== undefined) {
      const url = URL.createObjectURL(blob);
      link.setAttribute('href', url);
      link.setAttribute('download', filename);
      link.style.visibility = 'hidden';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(url);
    } else {
      // Fallback for older browsers
      window.open(`data:${mimeTypes[format]},${encodeURIComponent(content)}`);
    }
  }

  /**
   * Generate filename with timestamp
   */
  static generateFilename(baseName: string, format: string): string {
    const timestamp = new Date().toISOString().slice(0, 19).replace(/:/g, '-');
    return `${baseName}_${timestamp}.${format}`;
  }

  /**
   * Export table data with automatic column detection
   */
  static exportTableData<T>(
    data: T[],
    options: PartialExportOptions = { format: 'csv' }
  ): void {
    if (data.length === 0) {
      throw new Error('No data to export');
    }

    const sampleItem = data[0] as any;
    const columns: ExportColumn[] = Object.keys(sampleItem).map(key => ({
      key,
      header: this.formatHeader(key),
      type: this.detectColumnType(sampleItem[key]),
      formatter: this.getDefaultFormatter(key, sampleItem[key]),
    }));

    const opts = { ...this.DEFAULT_OPTIONS, ...options };
    let content: string;
    let extension: string;

    switch (opts.format) {
      case 'csv':
        content = this.exportToCSV(data, columns, opts);
        extension = 'csv';
        break;
      case 'json':
        content = this.exportToJSON(data, columns, opts);
        extension = 'json';
        break;
      default:
        throw new Error(`Unsupported export format: ${opts.format}`);
    }

    const filename = opts.filename || this.generateFilename('export', extension);
    this.downloadExport(content, filename, opts.format);
  }

  /**
   * Export specific columns with custom formatting
   */
  static exportCustomColumns<T>(
    data: T[],
    columns: ExportColumn[],
    options: PartialExportOptions = { format: 'csv' }
  ): void {
    const opts = { ...this.DEFAULT_OPTIONS, ...options };
    let content: string;
    let extension: string;

    switch (opts.format) {
      case 'csv':
        content = this.exportToCSV(data, columns, opts);
        extension = 'csv';
        break;
      case 'json':
        content = this.exportToJSON(data, columns, opts);
        extension = 'json';
        break;
      default:
        throw new Error(`Unsupported export format: ${opts.format}`);
    }

    const filename = opts.filename || this.generateFilename('export', extension);
    this.downloadExport(content, filename, opts.format);
  }

  // Private helper methods

  private static filterColumns(columns: ExportColumn[], excludeColumns?: string[]): ExportColumn[] {
    if (!excludeColumns) return columns;
    return columns.filter(col => !excludeColumns.includes(col.key));
  }

  private static getHeader(column: ExportColumn, customHeaders?: Record<string, string>): string {
    return customHeaders?.[column.key] || column.header;
  }

  private static formatValue(value: any, column: ExportColumn, options: ExportOptions): string {
    if (value == null || value === undefined) {
      return '';
    }

    // Use custom formatter if provided
    if (column.formatter) {
      return column.formatter(value);
    }

    // Use default formatters based on column type
    switch (column.type) {
      case 'date':
        return this.formatDate(value, options.dateFormat);
      case 'number':
        return this.formatNumber(value, options.numberFormat);
      case 'boolean':
        return value ? 'Yes' : 'No';
      default:
        return String(value);
    }
  }

  private static formatDate(value: any, dateFormat?: string): string {
    if (!value) return '';
    
    try {
      const date = new Date(value);
      if (isNaN(date.getTime())) return String(value);
      
      switch (dateFormat) {
        case 'ISO':
          return date.toISOString();
        case 'local':
          return date.toLocaleDateString();
        case 'short':
          return date.toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'short',
            day: 'numeric',
          });
        default:
          return formatTimestamp(date.getTime());
      }
    } catch {
      return String(value);
    }
  }

  private static formatNumber(value: any, numberFormat?: string): string {
    if (value == null || value === undefined) return '';
    
    const num = Number(value);
    if (isNaN(num)) return String(value);
    
    switch (numberFormat) {
      case 'locale':
        return formatNumber(num);
      case 'raw':
        return String(num);
      default:
        return formatNumber(num);
    }
  }

  private static escapeCSVValue(value: string): string {
    if (value.includes(',') || value.includes('"') || value.includes('\n')) {
      return `"${value.replace(/"/g, '""')}"`;
    }
    return value;
  }

  private static formatHeader(key: string): string {
    return key
      .replace(/([A-Z])/g, ' $1')
      .replace(/^./, str => str.toUpperCase())
      .trim();
  }

  private static detectColumnType(value: any): 'string' | 'number' | 'date' | 'boolean' {
    if (value == null || value === undefined) return 'string';
    
    if (typeof value === 'boolean') return 'boolean';
    if (typeof value === 'number') return 'number';
    
    // Try to detect date
    if (typeof value === 'string' || typeof value === 'number') {
      const date = new Date(value);
      if (!isNaN(date.getTime())) return 'date';
    }
    
    return 'string';
  }

  private static getDefaultFormatter(key: string, sampleValue: any): ((value: any) => string) | undefined {
    const type = this.detectColumnType(sampleValue);
    
    switch (type) {
      case 'date':
        return (value) => this.formatDate(value, 'short');
      case 'number':
        if (key.toLowerCase().includes('price') || key.toLowerCase().includes('amount')) {
          return (value) => formatPrice(Number(value));
        }
        if (key.toLowerCase().includes('percent') || key.toLowerCase().includes('rate')) {
          return (value) => `${formatNumber(Number(value))}%`;
        }
        return (value) => formatNumber(Number(value));
      case 'boolean':
        return (value) => value ? 'Yes' : 'No';
      default:
        return undefined;
    }
  }
}

// Convenience functions for common export scenarios

export const exportTransactions = (transactions: any[], options?: ExportOptions) => {
  const columns: ExportColumn[] = [
    { key: 'id', header: 'Transaction ID', type: 'string' },
    { key: 'hash', header: 'Hash', type: 'string' },
    { key: 'from', header: 'From Address', type: 'string' },
    { key: 'to', header: 'To Address', type: 'string' },
    { key: 'amount', header: 'Amount', type: 'number', formatter: (value) => formatPrice(Number(value)) },
    { key: 'asset', header: 'Asset', type: 'string' },
    { key: 'timestamp', header: 'Timestamp', type: 'date', formatter: (value) => formatTimestamp(value) },
    { key: 'status', header: 'Status', type: 'string' },
    { key: 'type', header: 'Type', type: 'string' },
  ];

  DataExporter.exportCustomColumns(transactions, columns, options);
};

export const exportBlocks = (blocks: any[], options?: ExportOptions) => {
  const columns: ExportColumn[] = [
    { key: 'id', header: 'Block ID', type: 'string' },
    { key: 'number', header: 'Block Number', type: 'number' },
    { key: 'hash', header: 'Hash', type: 'string' },
    { key: 'timestamp', header: 'Timestamp', type: 'date', formatter: (value) => formatTimestamp(value) },
    { key: 'validator', header: 'Validator', type: 'string' },
    { key: 'transactionCount', header: 'Transaction Count', type: 'number' },
    { key: 'round', header: 'Round', type: 'number' },
  ];

  DataExporter.exportCustomColumns(blocks, columns, options);
};

export const exportTrades = (trades: any[], options?: ExportOptions) => {
  const columns: ExportColumn[] = [
    { key: 'id', header: 'Trade ID', type: 'string' },
    { key: 'pair', header: 'Trading Pair', type: 'string' },
    { key: 'price', header: 'Price', type: 'number', formatter: (value) => formatPrice(Number(value)) },
    { key: 'amount', header: 'Amount', type: 'number', formatter: (value) => formatNumber(Number(value)) },
    { key: 'side', header: 'Side', type: 'string' },
    { key: 'timestamp', header: 'Timestamp', type: 'date', formatter: (value) => formatTimestamp(value) },
    { key: 'maker', header: 'Maker', type: 'string' },
    { key: 'taker', header: 'Taker', type: 'string' },
    { key: 'fee', header: 'Fee', type: 'number', formatter: (value) => formatPrice(Number(value)) },
  ];

  DataExporter.exportCustomColumns(trades, columns, options);
};

export const exportValidators = (validators: any[], options?: ExportOptions) => {
  const columns: ExportColumn[] = [
    { key: 'id', header: 'Validator ID', type: 'string' },
    { key: 'address', header: 'Address', type: 'string' },
    { key: 'stake', header: 'Stake', type: 'number', formatter: (value) => formatNumber(Number(value)) },
    { key: 'status', header: 'Status', type: 'string' },
    { key: 'lastSeen', header: 'Last Seen', type: 'date', formatter: (value) => formatTimestamp(value) },
    { key: 'pingLatency', header: 'Ping Latency', type: 'number', formatter: (value) => `${formatNumber(Number(value))}ms` },
  ];

  DataExporter.exportCustomColumns(validators, columns, options);
}; 