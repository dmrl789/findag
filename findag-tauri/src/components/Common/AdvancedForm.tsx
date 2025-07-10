import React, { useState, useEffect, useCallback } from 'react';
import { showNotification } from './NotificationSystem';

interface FormField {
  id: string;
  label: string;
  type: 'text' | 'email' | 'password' | 'number' | 'select' | 'textarea' | 'checkbox' | 'radio';
  required?: boolean;
  placeholder?: string;
  options?: { label: string; value: string }[];
  validation?: {
    pattern?: RegExp;
    minLength?: number;
    maxLength?: number;
    min?: number;
    max?: number;
    custom?: (value: any) => string | null;
  };
  defaultValue?: any;
}

interface FormStep {
  id: string;
  title: string;
  description?: string;
  fields: FormField[];
}

interface AdvancedFormProps {
  steps: FormStep[];
  onSubmit: (data: Record<string, any>) => void;
  onSave?: (data: Record<string, any>) => void;
  autoSave?: boolean;
  autoSaveInterval?: number;
  showProgress?: boolean;
  className?: string;
}

const AdvancedForm: React.FC<AdvancedFormProps> = ({
  steps,
  onSubmit,
  onSave,
  autoSave = false,
  autoSaveInterval = 30000,
  showProgress = true,
  className = '',
}) => {
  const [currentStep, setCurrentStep] = useState(0);
  const [formData, setFormData] = useState<Record<string, any>>({});
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [lastSaved, setLastSaved] = useState<Date | null>(null);

  // Initialize form data with default values
  useEffect(() => {
    const initialData: Record<string, any> = {};
    steps.forEach(step => {
      step.fields.forEach(field => {
        if (field.defaultValue !== undefined) {
          initialData[field.id] = field.defaultValue;
        }
      });
    });
    setFormData(initialData);
  }, [steps]);

  // Auto-save functionality
  useEffect(() => {
    if (!autoSave || !onSave) return;

    const interval = setInterval(() => {
      if (Object.keys(formData).length > 0) {
        handleAutoSave();
      }
    }, autoSaveInterval);

    return () => clearInterval(interval);
  }, [autoSave, autoSaveInterval, formData, onSave]);

  const validateField = useCallback((field: FormField, value: any): string | null => {
    if (field.required && (!value || (typeof value === 'string' && value.trim() === ''))) {
      return `${field.label} is required`;
    }

    if (!value) return null;

    const validation = field.validation;
    if (!validation) return null;

    if (validation.pattern && !validation.pattern.test(value)) {
      return `${field.label} format is invalid`;
    }

    if (validation.minLength && value.length < validation.minLength) {
      return `${field.label} must be at least ${validation.minLength} characters`;
    }

    if (validation.maxLength && value.length > validation.maxLength) {
      return `${field.label} must be no more than ${validation.maxLength} characters`;
    }

    if (validation.min !== undefined && Number(value) < validation.min) {
      return `${field.label} must be at least ${validation.min}`;
    }

    if (validation.max !== undefined && Number(value) > validation.max) {
      return `${field.label} must be no more than ${validation.max}`;
    }

    if (validation.custom) {
      return validation.custom(value);
    }

    return null;
  }, []);

  const validateStep = useCallback((stepIndex: number): boolean => {
    const step = steps[stepIndex];
    const newErrors: Record<string, string> = {};

    step.fields.forEach(field => {
      const error = validateField(field, formData[field.id]);
      if (error) {
        newErrors[field.id] = error;
      }
    });

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  }, [steps, formData, validateField]);

  const handleFieldChange = (fieldId: string, value: any) => {
    setFormData(prev => ({ ...prev, [fieldId]: value }));
    
    // Clear error for this field
    if (errors[fieldId]) {
      setErrors(prev => {
        const newErrors = { ...prev };
        delete newErrors[fieldId];
        return newErrors;
      });
    }
  };

  const handleAutoSave = async () => {
    if (!onSave) return;

    setIsSaving(true);
    try {
      await onSave(formData);
      setLastSaved(new Date());
      showNotification({
        type: 'success',
        title: 'Auto-saved',
        message: 'Form data has been automatically saved',
      });
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Auto-save failed',
        message: 'Failed to auto-save form data',
      });
    } finally {
      setIsSaving(false);
    }
  };

  const handleNext = () => {
    if (validateStep(currentStep)) {
      if (currentStep < steps.length - 1) {
        setCurrentStep(currentStep + 1);
      } else {
        handleSubmit();
      }
    }
  };

  const handlePrevious = () => {
    if (currentStep > 0) {
      setCurrentStep(currentStep - 1);
    }
  };

  const handleSubmit = async () => {
    if (!validateStep(currentStep)) return;

    setIsSubmitting(true);
    try {
      await onSubmit(formData);
      showNotification({
        type: 'success',
        title: 'Success',
        message: 'Form submitted successfully',
      });
    } catch (error) {
      showNotification({
        type: 'error',
        title: 'Submission failed',
        message: 'Failed to submit form',
      });
    } finally {
      setIsSubmitting(false);
    }
  };

  const renderField = (field: FormField) => {
    const value = formData[field.id] || '';
    const error = errors[field.id];

    const commonProps = {
      id: field.id,
      value: value,
      onChange: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => 
        handleFieldChange(field.id, e.target.value),
      className: `w-full px-3 py-2 border rounded-md text-sm ${
        error
          ? 'border-red-500 bg-red-50 dark:bg-red-900/20'
          : 'border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800'
      } text-gray-900 dark:text-white`,
      placeholder: field.placeholder,
    };

    switch (field.type) {
      case 'textarea':
        return (
          <textarea
            {...commonProps}
            rows={4}
            onChange={(e) => handleFieldChange(field.id, e.target.value)}
          />
        );

      case 'select':
        return (
          <select {...commonProps}>
            <option value="">Select {field.label}</option>
            {field.options?.map(option => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        );

      case 'checkbox':
        return (
          <div className="flex items-center">
            <input
              type="checkbox"
              id={field.id}
              checked={Boolean(value)}
              onChange={(e) => handleFieldChange(field.id, e.target.checked)}
              className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <label htmlFor={field.id} className="ml-2 text-sm text-gray-700 dark:text-gray-300">
              {field.label}
            </label>
          </div>
        );

      case 'radio':
        return (
          <div className="space-y-2">
            {field.options?.map(option => (
              <div key={option.value} className="flex items-center">
                <input
                  type="radio"
                  id={`${field.id}-${option.value}`}
                  name={field.id}
                  value={option.value}
                  checked={value === option.value}
                  onChange={(e) => handleFieldChange(field.id, e.target.value)}
                  className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300"
                />
                <label htmlFor={`${field.id}-${option.value}`} className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                  {option.label}
                </label>
              </div>
            ))}
          </div>
        );

      default:
        return (
          <input
            {...commonProps}
            type={field.type}
          />
        );
    }
  };

  const currentStepData = steps[currentStep];

  return (
    <div className={`max-w-2xl mx-auto ${className}`}>
      {/* Progress Bar */}
      {showProgress && (
        <div className="mb-8">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
              Step {currentStep + 1} of {steps.length}
            </span>
            <span className="text-sm text-gray-500 dark:text-gray-400">
              {Math.round(((currentStep + 1) / steps.length) * 100)}% Complete
            </span>
          </div>
          <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
            <div
              className="bg-blue-600 h-2 rounded-full transition-all duration-300"
              style={{ width: `${((currentStep + 1) / steps.length) * 100}%` }}
            />
          </div>
        </div>
      )}

      {/* Step Title */}
      <div className="mb-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          {currentStepData.title}
        </h2>
        {currentStepData.description && (
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {currentStepData.description}
          </p>
        )}
      </div>

      {/* Form Fields */}
      <div className="space-y-6">
        {currentStepData.fields.map(field => (
          <div key={field.id}>
            <label htmlFor={field.id} className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              {field.label}
              {field.required && <span className="text-red-500 ml-1">*</span>}
            </label>
            {renderField(field)}
            {errors[field.id] && (
              <p className="mt-1 text-sm text-red-600 dark:text-red-400">
                {errors[field.id]}
              </p>
            )}
          </div>
        ))}
      </div>

      {/* Navigation Buttons */}
      <div className="flex items-center justify-between mt-8">
        <div className="flex items-center space-x-4">
          {currentStep > 0 && (
            <button
              type="button"
              onClick={handlePrevious}
              className="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              Previous
            </button>
          )}
          
          {autoSave && onSave && (
            <button
              type="button"
              onClick={handleAutoSave}
              disabled={isSaving}
              className="px-4 py-2 text-sm font-medium text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md hover:bg-blue-100 dark:hover:bg-blue-900/40 disabled:opacity-50"
            >
              {isSaving ? 'Saving...' : 'Save'}
            </button>
          )}
        </div>

        <div className="flex items-center space-x-4">
          {lastSaved && (
            <span className="text-xs text-gray-500 dark:text-gray-400">
              Last saved: {lastSaved.toLocaleTimeString()}
            </span>
          )}
          
          <button
            type="button"
            onClick={handleNext}
            disabled={isSubmitting}
            className="px-6 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
          >
            {currentStep === steps.length - 1
              ? (isSubmitting ? 'Submitting...' : 'Submit')
              : 'Next'}
          </button>
        </div>
      </div>
    </div>
  );
};

export default AdvancedForm; 