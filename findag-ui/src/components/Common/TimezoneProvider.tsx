import React, { createContext, useContext, useEffect, useState } from 'react';
import { format, formatInTimeZone, utcToZonedTime, zonedTimeToUtc } from 'date-fns-tz';
import { addMinutes, subMinutes, startOfDay, endOfDay } from 'date-fns';

export interface Timezone {
  value: string;
  label: string;
  offset: string;
  description: string;
}

export interface TimezoneState {
  // Current timezone
  timezone: string;
  setTimezone: (timezone: string) => void;
  
  // Available timezones
  availableTimezones: Timezone[];
  
  // Utility functions
  formatInTimezone: (date: Date, formatString: string) => string;
  formatInCurrentTimezone: (date: Date, formatString: string) => string;
  convertToTimezone: (date: Date, targetTimezone: string) => Date;
  convertFromTimezone: (date: Date, sourceTimezone: string) => Date;
  getTimezoneOffset: (timezone: string) => number;
  getCurrentTimezoneOffset: () => number;
  
  // Chart utilities
  formatChartTime: (timestamp: number, formatString?: string) => string;
  formatChartTooltip: (timestamp: number, value: any) => string;
  getTimeRange: (timeFrame: string, timezone?: string) => { start: Date; end: Date };
  
  // Data utilities
  convertDataToTimezone: <T extends { timestamp: number }>(data: T[], timezone: string) => T[];
  convertDataFromTimezone: <T extends { timestamp: number }>(data: T[], timezone: string) => T[];
}

const TimezoneContext = createContext<TimezoneState | undefined>(undefined);

export const useTimezone = () => {
  const context = useContext(TimezoneContext);
  if (!context) {
    throw new Error('useTimezone must be used within a TimezoneProvider');
  }
  return context;
};

// Common timezones with their offsets
const COMMON_TIMEZONES: Timezone[] = [
  { value: 'UTC', label: 'UTC', offset: '+00:00', description: 'Coordinated Universal Time' },
  { value: 'America/New_York', label: 'Eastern Time', offset: '-05:00', description: 'Eastern Standard Time' },
  { value: 'America/Chicago', label: 'Central Time', offset: '-06:00', description: 'Central Standard Time' },
  { value: 'America/Denver', label: 'Mountain Time', offset: '-07:00', description: 'Mountain Standard Time' },
  { value: 'America/Los_Angeles', label: 'Pacific Time', offset: '-08:00', description: 'Pacific Standard Time' },
  { value: 'Europe/London', label: 'London', offset: '+00:00', description: 'Greenwich Mean Time' },
  { value: 'Europe/Paris', label: 'Paris', offset: '+01:00', description: 'Central European Time' },
  { value: 'Europe/Berlin', label: 'Berlin', offset: '+01:00', description: 'Central European Time' },
  { value: 'Asia/Tokyo', label: 'Tokyo', offset: '+09:00', description: 'Japan Standard Time' },
  { value: 'Asia/Shanghai', label: 'Shanghai', offset: '+08:00', description: 'China Standard Time' },
  { value: 'Asia/Singapore', label: 'Singapore', offset: '+08:00', description: 'Singapore Time' },
  { value: 'Australia/Sydney', label: 'Sydney', offset: '+10:00', description: 'Australian Eastern Time' },
  { value: 'Pacific/Auckland', label: 'Auckland', offset: '+12:00', description: 'New Zealand Standard Time' },
];

interface TimezoneProviderProps {
  children: React.ReactNode;
  defaultTimezone?: string;
}

export const TimezoneProvider: React.FC<TimezoneProviderProps> = ({ 
  children, 
  defaultTimezone = 'UTC' 
}) => {
  const [timezone, setTimezone] = useState<string>(defaultTimezone);
  const [availableTimezones, setAvailableTimezones] = useState<Timezone[]>(COMMON_TIMEZONES);

  // Load timezone from localStorage on mount
  useEffect(() => {
    const savedTimezone = localStorage.getItem('findag-timezone');
    if (savedTimezone) {
      setTimezone(savedTimezone);
    } else {
      // Try to detect user's timezone
      try {
        const userTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
        if (userTimezone && COMMON_TIMEZONES.some(tz => tz.value === userTimezone)) {
          setTimezone(userTimezone);
        }
      } catch (error) {
        console.warn('Could not detect user timezone:', error);
      }
    }
  }, []);

  // Save timezone to localStorage when it changes
  useEffect(() => {
    localStorage.setItem('findag-timezone', timezone);
  }, [timezone]);

  // Update timezone offsets based on current date (for DST)
  useEffect(() => {
    const updateTimezones = () => {
      const updatedTimezones = COMMON_TIMEZONES.map(tz => {
        try {
          const now = new Date();
          const offset = formatInTimeZone(now, tz.value, 'xxx');
          return {
            ...tz,
            offset,
            description: `${tz.description} (${offset})`
          };
        } catch (error) {
          return tz;
        }
      });
      setAvailableTimezones(updatedTimezones);
    };

    updateTimezones();
    // Update every hour to handle DST changes
    const interval = setInterval(updateTimezones, 60 * 60 * 1000);
    return () => clearInterval(interval);
  }, []);

  const formatInTimezone = (date: Date, formatString: string): string => {
    try {
      return formatInTimeZone(date, timezone, formatString);
    } catch (error) {
      console.warn('Error formatting date in timezone:', error);
      return format(date, formatString);
    }
  };

  const formatInCurrentTimezone = (date: Date, formatString: string): string => {
    return formatInTimezone(date, formatString);
  };

  const convertToTimezone = (date: Date, targetTimezone: string): Date => {
    try {
      return utcToZonedTime(date, targetTimezone);
    } catch (error) {
      console.warn('Error converting to timezone:', error);
      return date;
    }
  };

  const convertFromTimezone = (date: Date, sourceTimezone: string): Date => {
    try {
      return zonedTimeToUtc(date, sourceTimezone);
    } catch (error) {
      console.warn('Error converting from timezone:', error);
      return date;
    }
  };

  const getTimezoneOffset = (timezoneValue: string): number => {
    try {
      const now = new Date();
      const utc = formatInTimeZone(now, 'UTC', 'yyyy-MM-dd HH:mm:ss');
      const tz = formatInTimeZone(now, timezoneValue, 'yyyy-MM-dd HH:mm:ss');
      const utcDate = new Date(utc);
      const tzDate = new Date(tz);
      return (tzDate.getTime() - utcDate.getTime()) / (1000 * 60);
    } catch (error) {
      console.warn('Error getting timezone offset:', error);
      return 0;
    }
  };

  const getCurrentTimezoneOffset = (): number => {
    return getTimezoneOffset(timezone);
  };

  const formatChartTime = (timestamp: number, formatString: string = 'MMM dd, HH:mm'): string => {
    const date = new Date(timestamp);
    return formatInTimezone(date, formatString);
  };

  const formatChartTooltip = (timestamp: number, value: any): string => {
    const date = new Date(timestamp);
    const timeString = formatInTimezone(date, 'MMM dd, yyyy HH:mm:ss');
    return `${timeString}\nValue: ${value}`;
  };

  const getTimeRange = (timeFrame: string, targetTimezone: string = timezone) => {
    const now = new Date();
    const end = endOfDay(convertToTimezone(now, targetTimezone));
    let start: Date;

    switch (timeFrame) {
      case '1m':
        start = subMinutes(now, 1);
        break;
      case '5m':
        start = subMinutes(now, 5);
        break;
      case '15m':
        start = subMinutes(now, 15);
        break;
      case '1h':
        start = subMinutes(now, 60);
        break;
      case '4h':
        start = subMinutes(now, 240);
        break;
      case '1d':
        start = startOfDay(convertToTimezone(now, targetTimezone));
        break;
      case '1w':
        start = subMinutes(now, 7 * 24 * 60);
        break;
      default:
        start = subMinutes(now, 60);
    }

    return {
      start: convertToTimezone(start, targetTimezone),
      end: convertToTimezone(end, targetTimezone)
    };
  };

  const convertDataToTimezone = <T extends { timestamp: number }>(data: T[], targetTimezone: string): T[] => {
    return data.map(item => ({
      ...item,
      timestamp: convertToTimezone(new Date(item.timestamp), targetTimezone).getTime()
    }));
  };

  const convertDataFromTimezone = <T extends { timestamp: number }>(data: T[], sourceTimezone: string): T[] => {
    return data.map(item => ({
      ...item,
      timestamp: convertFromTimezone(new Date(item.timestamp), sourceTimezone).getTime()
    }));
  };

  const value: TimezoneState = {
    timezone,
    setTimezone,
    availableTimezones,
    formatInTimezone,
    formatInCurrentTimezone,
    convertToTimezone,
    convertFromTimezone,
    getTimezoneOffset,
    getCurrentTimezoneOffset,
    formatChartTime,
    formatChartTooltip,
    getTimeRange,
    convertDataToTimezone,
    convertDataFromTimezone,
  };

  return (
    <TimezoneContext.Provider value={value}>
      {children}
    </TimezoneContext.Provider>
  );
};

// Timezone selector component
interface TimezoneSelectorProps {
  value: string;
  onChange: (timezone: string) => void;
  className?: string;
  showDescription?: boolean;
}

export const TimezoneSelector: React.FC<TimezoneSelectorProps> = ({
  value,
  onChange,
  className = '',
  showDescription = false,
}) => {
  const { availableTimezones } = useTimezone();

  return (
    <div className={className}>
      <select
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent"
      >
        {availableTimezones.map((tz) => (
          <option key={tz.value} value={tz.value}>
            {tz.label} ({tz.offset})
            {showDescription && ` - ${tz.description}`}
          </option>
        ))}
      </select>
    </div>
  );
};

// Timezone display component
interface TimezoneDisplayProps {
  timezone?: string;
  showOffset?: boolean;
  className?: string;
}

export const TimezoneDisplay: React.FC<TimezoneDisplayProps> = ({
  timezone,
  showOffset = true,
  className = '',
}) => {
  const { timezone: currentTimezone, availableTimezones } = useTimezone();
  const tz = timezone || currentTimezone;
  const timezoneInfo = availableTimezones.find(t => t.value === tz);

  if (!timezoneInfo) {
    return <span className={className}>Unknown timezone</span>;
  }

  return (
    <span className={className}>
      {timezoneInfo.label}
      {showOffset && ` (${timezoneInfo.offset})`}
    </span>
  );
};

// Current time display component
interface CurrentTimeDisplayProps {
  timezone?: string;
  format?: string;
  className?: string;
  updateInterval?: number;
}

export const CurrentTimeDisplay: React.FC<CurrentTimeDisplayProps> = ({
  timezone,
  format = 'HH:mm:ss',
  className = '',
  updateInterval = 1000,
}) => {
  const { formatInCurrentTimezone } = useTimezone();
  const [currentTime, setCurrentTime] = useState(new Date());

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentTime(new Date());
    }, updateInterval);

    return () => clearInterval(interval);
  }, [updateInterval]);

  const displayTimezone = timezone || 'current';
  const timeString = displayTimezone === 'current' 
    ? formatInCurrentTimezone(currentTime, format)
    : formatInTimeZone(currentTime, timezone!, format);

  return (
    <span className={className}>
      {timeString}
    </span>
  );
}; 