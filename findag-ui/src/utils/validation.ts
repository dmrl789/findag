import React from 'react';

export interface ValidationRule {
  required?: boolean;
  minLength?: number;
  maxLength?: number;
  pattern?: RegExp;
  custom?: (value: any) => string | null;
  email?: boolean;
  url?: boolean;
  numeric?: boolean;
  positive?: boolean;
  range?: { min: number; max: number };
  match?: { field: string; message: string };
}

export interface ValidationResult {
  isValid: boolean;
  errors: string[];
}

export interface FieldValidation {
  value: any;
  rules: ValidationRule;
  fieldName?: string;
}

// Common validation patterns
export const PATTERNS = {
  EMAIL: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  URL: /^https?:\/\/.+/,
  PHONE: /^\+?[\d\s\-\(\)]+$/,
  ALPHANUMERIC: /^[a-zA-Z0-9]+$/,
  ALPHANUMERIC_WITH_SPACES: /^[a-zA-Z0-9\s]+$/,
  DECIMAL: /^\d+(\.\d+)?$/,
  INTEGER: /^\d+$/,
  HEX_COLOR: /^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/,
  IP_ADDRESS: /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/,
  CRYPTO_ADDRESS: /^[13][a-km-zA-HJ-NP-Z1-9]{25,34}$/,
  TRANSACTION_HASH: /^[a-fA-F0-9]{64}$/,
};

// Validation functions
export function validateField(value: any, rules: ValidationRule, fieldName?: string): string[] {
  const errors: string[] = [];
  const field = fieldName || 'Field';

  // Required validation
  if (rules.required && (value === null || value === undefined || value === '')) {
    errors.push(`${field} is required`);
    return errors; // Don't continue with other validations if required field is empty
  }

  // Skip other validations if value is empty and not required
  if (value === null || value === undefined || value === '') {
    return errors;
  }

  const stringValue = String(value);

  // Min length validation
  if (rules.minLength && stringValue.length < rules.minLength) {
    errors.push(`${field} must be at least ${rules.minLength} characters long`);
  }

  // Max length validation
  if (rules.maxLength && stringValue.length > rules.maxLength) {
    errors.push(`${field} must be no more than ${rules.maxLength} characters long`);
  }

  // Pattern validation
  if (rules.pattern && !rules.pattern.test(stringValue)) {
    errors.push(`${field} format is invalid`);
  }

  // Email validation
  if (rules.email && !PATTERNS.EMAIL.test(stringValue)) {
    errors.push(`${field} must be a valid email address`);
  }

  // URL validation
  if (rules.url && !PATTERNS.URL.test(stringValue)) {
    errors.push(`${field} must be a valid URL`);
  }

  // Numeric validation
  if (rules.numeric) {
    const numValue = Number(value);
    if (isNaN(numValue)) {
      errors.push(`${field} must be a number`);
    } else {
      // Positive validation
      if (rules.positive && numValue <= 0) {
        errors.push(`${field} must be positive`);
      }

      // Range validation
      if (rules.range) {
        if (numValue < rules.range.min) {
          errors.push(`${field} must be at least ${rules.range.min}`);
        }
        if (numValue > rules.range.max) {
          errors.push(`${field} must be no more than ${rules.range.max}`);
        }
      }
    }
  }

  // Custom validation
  if (rules.custom) {
    const customError = rules.custom(value);
    if (customError) {
      errors.push(customError);
    }
  }

  return errors;
}

// Validate multiple fields
export function validateForm(fields: Record<string, FieldValidation>): ValidationResult {
  const errors: string[] = [];
  const fieldErrors: Record<string, string[]> = {};

  // Validate each field
  Object.entries(fields).forEach(([fieldName, field]) => {
    const fieldValidationErrors = validateField(field.value, field.rules, field.fieldName || fieldName);
    if (fieldValidationErrors.length > 0) {
      fieldErrors[fieldName] = fieldValidationErrors;
      errors.push(...fieldValidationErrors);
    }
  });

  // Check for field matching (e.g., password confirmation)
  Object.entries(fields).forEach(([fieldName, field]) => {
    if (field.rules.match) {
      const matchField = fields[field.rules.match.field];
      if (matchField && field.value !== matchField.value) {
        const error = field.rules.match.message;
        fieldErrors[fieldName] = [...(fieldErrors[fieldName] || []), error];
        errors.push(error);
      }
    }
  });

  return {
    isValid: errors.length === 0,
    errors,
  };
}

// Common validation rules
export const VALIDATION_RULES = {
  REQUIRED: { required: true },
  EMAIL: { required: true, email: true },
  PASSWORD: { 
    required: true, 
    minLength: 8, 
    pattern: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]/,
    custom: (value: string) => {
      if (!/(?=.*[a-z])/.test(value)) return 'Password must contain at least one lowercase letter';
      if (!/(?=.*[A-Z])/.test(value)) return 'Password must contain at least one uppercase letter';
      if (!/(?=.*\d)/.test(value)) return 'Password must contain at least one number';
      if (!/(?=.*[@$!%*?&])/.test(value)) return 'Password must contain at least one special character';
      return null;
    }
  },
  USERNAME: { 
    required: true, 
    minLength: 3, 
    maxLength: 20, 
    pattern: PATTERNS.ALPHANUMERIC 
  },
  URL: { required: true, url: true },
  PHONE: { required: true, pattern: PATTERNS.PHONE },
  POSITIVE_NUMBER: { required: true, numeric: true, positive: true },
  AMOUNT: { 
    required: true, 
    numeric: true, 
    positive: true,
    range: { min: 0.000001, max: 999999999 }
  },
  TRANSACTION_HASH: { required: true, pattern: PATTERNS.TRANSACTION_HASH },
  CRYPTO_ADDRESS: { required: true, pattern: PATTERNS.CRYPTO_ADDRESS },
  IP_ADDRESS: { required: true, pattern: PATTERNS.IP_ADDRESS },
};

// React hook for form validation
export function useFormValidation<T extends Record<string, any>>(
  initialValues: T,
  validationRules: Record<keyof T, ValidationRule>
) {
  const [values, setValues] = React.useState<T>(initialValues);
  const [errors, setErrors] = React.useState<Record<keyof T, string[]>>({} as any);
  const [touched, setTouched] = React.useState<Record<keyof T, boolean>>({} as any);

  const validateFieldValue = React.useCallback((fieldName: keyof T, value: any) => {
    const fieldErrors = validateField(value, validationRules[fieldName], String(fieldName));
    setErrors(prev => ({
      ...prev,
      [fieldName]: fieldErrors,
    }));
    return fieldErrors.length === 0;
  }, [validationRules]);

  const handleChange = React.useCallback((fieldName: keyof T, value: any) => {
    setValues(prev => ({
      ...prev,
      [fieldName]: value,
    }));
    
    if (touched[fieldName]) {
      validateFieldValue(fieldName, value);
    }
  }, [touched, validateFieldValue]);

  const handleBlur = React.useCallback((fieldName: keyof T) => {
    setTouched(prev => ({
      ...prev,
      [fieldName]: true,
    }));
    validateFieldValue(fieldName, values[fieldName]);
  }, [values, validateFieldValue]);

  const validateForm = React.useCallback(() => {
    const fields: Record<string, FieldValidation> = {};
    Object.keys(validationRules).forEach(key => {
      fields[key] = {
        value: values[key],
        rules: validationRules[key as keyof T],
        fieldName: key,
      };
    });

    const result = validateForm(fields);
    const fieldErrors: Record<keyof T, string[]> = {} as any;
    
    Object.keys(validationRules).forEach(key => {
      fieldErrors[key as keyof T] = result.errors.filter(error => 
        error.includes(key) || error.includes(validationRules[key as keyof T].fieldName || key)
      );
    });

    setErrors(fieldErrors);
    return result.isValid;
  }, [values, validationRules]);

  const reset = React.useCallback(() => {
    setValues(initialValues);
    setErrors({} as any);
    setTouched({} as any);
  }, [initialValues]);

  return {
    values,
    errors,
    touched,
    handleChange,
    handleBlur,
    validateForm,
    reset,
    setValues,
    setErrors,
    setTouched,
  };
}

// Utility functions
export function formatValidationError(fieldName: string, error: string): string {
  return error.replace(fieldName, fieldName.charAt(0).toUpperCase() + fieldName.slice(1));
}

export function hasErrors(errors: Record<string, string[]>): boolean {
  return Object.values(errors).some(fieldErrors => fieldErrors.length > 0);
}

export function getFirstError(errors: Record<string, string[]>): string | null {
  for (const fieldErrors of Object.values(errors)) {
    if (fieldErrors.length > 0) {
      return fieldErrors[0];
    }
  }
  return null;
} 