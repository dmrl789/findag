import React from 'react';
import { LucideIcon } from 'lucide-react';
import { clsx } from 'clsx';

interface MetricsCardProps {
  title: string;
  value: string | number;
  subtitle?: string;
  icon: LucideIcon;
  trend?: {
    value: number;
    isPositive: boolean;
  };
  color?: 'primary' | 'success' | 'warning' | 'danger' | 'gray';
  loading?: boolean;
}

const colorClasses = {
  primary: {
    bg: 'bg-primary-50',
    icon: 'text-primary-600',
    border: 'border-primary-200',
  },
  success: {
    bg: 'bg-success-50',
    icon: 'text-success-600',
    border: 'border-success-200',
  },
  warning: {
    bg: 'bg-warning-50',
    icon: 'text-warning-600',
    border: 'border-warning-200',
  },
  danger: {
    bg: 'bg-danger-50',
    icon: 'text-danger-600',
    border: 'border-danger-200',
  },
  gray: {
    bg: 'bg-gray-50',
    icon: 'text-gray-600',
    border: 'border-gray-200',
  },
};

export const MetricsCard: React.FC<MetricsCardProps> = ({
  title,
  value,
  subtitle,
  icon: Icon,
  trend,
  color = 'primary',
  loading = false,
}) => {
  const colors = colorClasses[color];

  if (loading) {
    return (
      <div className={clsx('card border', colors.border)}>
        <div className="animate-pulse">
          <div className="flex items-center justify-between mb-4">
            <div className="h-4 bg-gray-200 rounded w-24"></div>
            <div className="h-8 w-8 bg-gray-200 rounded"></div>
          </div>
          <div className="h-8 bg-gray-200 rounded w-20 mb-2"></div>
          <div className="h-3 bg-gray-200 rounded w-32"></div>
        </div>
      </div>
    );
  }

  return (
    <div className={clsx('card border', colors.border)}>
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-sm font-medium text-gray-600">{title}</h3>
        <div className={clsx('p-2 rounded-lg', colors.bg)}>
          <Icon className={clsx('h-5 w-5', colors.icon)} />
        </div>
      </div>
      
      <div className="flex items-baseline space-x-2">
        <p className="text-2xl font-semibold text-gray-900">{value}</p>
        {trend && (
          <span
            className={clsx(
              'text-sm font-medium',
              trend.isPositive ? 'text-success-600' : 'text-danger-600'
            )}
          >
            {trend.isPositive ? '+' : ''}{trend.value}%
          </span>
        )}
      </div>
      
      {subtitle && (
        <p className="text-sm text-gray-500 mt-1">{subtitle}</p>
      )}
    </div>
  );
}; 