import React, { useState, useEffect, useCallback } from 'react';
import { AlertCircle, CheckCircle } from 'lucide-react';
import { inputValidator, VALIDATION_SCHEMAS, ValidationSchema } from '../../utils/security';

interface ValidationState {
  [key: string]: {
    isValid: boolean;
    message?: string;
    isDirty: boolean;
  };
}

interface FormValidatorProps {
  children: React.ReactNode;
  schema: ValidationSchema;
  onSubmit?: (data: Record<string, any>) => void;
  onValidationChange?: (isValid: boolean) => void;
  showErrors?: boolean;
  validateOnChange?: boolean;
  validateOnBlur?: boolean;
}

export const FormValidator: React.FC<FormValidatorProps> = ({
  children,
  schema,
  onSubmit,
  onValidationChange,
  showErrors = true,
  validateOnChange = true,
  validateOnBlur = true
}) => {
  const [formData, setFormData] = useState<Record<string, any>>({});
  const [validationState, setValidationState] = useState<ValidationState>({});
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Initialize validation state
  useEffect(() => {
    const initialState: ValidationState = {};
    Object.keys(schema).forEach(field => {
      initialState[field] = {
        isValid: true,
        isDirty: false
      };
    });
    setValidationState(initialState);
  }, [schema]);

  // Validate a single field
  const validateField = useCallback((field: string, value: any): boolean => {
    const rule = schema[field];
    if (!rule) return true;

    const validation = inputValidator.validateField(value, rule);
    const isValid = validation.isValid;

    setValidationState(prev => ({
      ...prev,
      [field]: {
        isValid,
        message: validation.message,
        isDirty: true
      }
    }));

    return isValid;
  }, [schema]);

  // Validate entire form
  const validateForm = useCallback((): boolean => {
    const validation = inputValidator.validateForm(formData, schema);
    
    setValidationState(prev => {
      const newState = { ...prev };
      Object.keys(schema).forEach(field => {
        newState[field] = {
          isValid: !validation.errors[field],
          message: validation.errors[field],
          isDirty: true
        };
      });
      return newState;
    });

    return validation.isValid;
  }, [formData, schema]);

  // Handle field change
  const handleFieldChange = useCallback((field: string, value: any) => {
    setFormData(prev => ({ ...prev, [field]: value }));

    if (validateOnChange) {
      validateField(field, value);
    }
  }, [validateField, validateOnChange]);

  // Handle field blur
  const handleFieldBlur = useCallback((field: string, value: any) => {
    if (validateOnBlur) {
      validateField(field, value);
    }
  }, [validateField, validateOnBlur]);

  // Handle form submission
  const handleSubmit = useCallback(async (e: React.FormEvent) => {
    e.preventDefault();
    
    setIsSubmitting(true);
    
    try {
      const isValid = validateForm();
      
      if (isValid && onSubmit) {
        await onSubmit(formData);
      }
    } catch (error) {
      console.error('Form submission error:', error);
    } finally {
      setIsSubmitting(false);
    }
  }, [formData, validateForm, onSubmit]);

  // Check if form is valid
  const isFormValid = Object.values(validationState).every(field => field.isValid);

  // Notify parent of validation changes
  useEffect(() => {
    onValidationChange?.(isFormValid);
  }, [isFormValid, onValidationChange]);

  // Create context value
  const contextValue = {
    formData,
    validationState,
    isSubmitting,
    isFormValid,
    handleFieldChange,
    handleFieldBlur,
    validateField,
    validateForm,
    handleSubmit
  };

  return (
    <FormValidatorContext.Provider value={contextValue}>
      <form onSubmit={handleSubmit} className="space-y-4">
        {children}
      </form>
    </FormValidatorContext.Provider>
  );
};

// Context for form validation
interface FormValidatorContextType {
  formData: Record<string, any>;
  validationState: ValidationState;
  isSubmitting: boolean;
  isFormValid: boolean;
  handleFieldChange: (field: string, value: any) => void;
  handleFieldBlur: (field: string, value: any) => void;
  validateField: (field: string, value: any) => boolean;
  validateForm: () => boolean;
  handleSubmit: (e: React.FormEvent) => void;
}

const FormValidatorContext = React.createContext<FormValidatorContextType | null>(null);

// Hook to use form validation context
export const useFormValidator = () => {
  const context = React.useContext(FormValidatorContext);
  if (!context) {
    throw new Error('useFormValidator must be used within a FormValidator');
  }
  return context;
};

// Validated input component
interface ValidatedInputProps {
  name: string;
  type?: string;
  placeholder?: string;
  label?: string;
  required?: boolean;
  className?: string;
  disabled?: boolean;
  autoComplete?: string;
}

export const ValidatedInput: React.FC<ValidatedInputProps> = ({
  name,
  type = 'text',
  placeholder,
  label,
  required = false,
  className = '',
  disabled = false,
  autoComplete
}) => {
  const {
    formData,
    validationState,
    handleFieldChange,
    handleFieldBlur
  } = useFormValidator();

  const fieldState = validationState[name] || { isValid: true, isDirty: false };
  const value = formData[name] || '';
  const showError = fieldState.isDirty && !fieldState.isValid;

  return (
    <div className="space-y-1">
      {label && (
        <label htmlFor={name} className="block text-sm font-medium text-gray-700">
          {label}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
      )}
      
      <div className="relative">
        <input
          id={name}
          name={name}
          type={type}
          value={value}
          onChange={(e) => handleFieldChange(name, e.target.value)}
          onBlur={(e) => handleFieldBlur(name, e.target.value)}
          placeholder={placeholder}
          disabled={disabled}
          autoComplete={autoComplete}
          className={`
            w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500
            ${showError ? 'border-red-300 focus:ring-red-500' : 'border-gray-300'}
            ${disabled ? 'bg-gray-100 cursor-not-allowed' : ''}
            ${className}
          `}
        />
        
        {showError && (
          <div className="absolute inset-y-0 right-0 flex items-center pr-3">
            <AlertCircle className="w-5 h-5 text-red-500" />
          </div>
        )}
        
        {fieldState.isDirty && fieldState.isValid && (
          <div className="absolute inset-y-0 right-0 flex items-center pr-3">
            <CheckCircle className="w-5 h-5 text-green-500" />
          </div>
        )}
      </div>
      
      {showError && fieldState.message && (
        <p className="text-sm text-red-600">{fieldState.message}</p>
      )}
    </div>
  );
};

// Validated textarea component
interface ValidatedTextareaProps {
  name: string;
  placeholder?: string;
  label?: string;
  required?: boolean;
  rows?: number;
  className?: string;
  disabled?: boolean;
}

export const ValidatedTextarea: React.FC<ValidatedTextareaProps> = ({
  name,
  placeholder,
  label,
  required = false,
  rows = 3,
  className = '',
  disabled = false
}) => {
  const {
    formData,
    validationState,
    handleFieldChange,
    handleFieldBlur
  } = useFormValidator();

  const fieldState = validationState[name] || { isValid: true, isDirty: false };
  const value = formData[name] || '';
  const showError = fieldState.isDirty && !fieldState.isValid;

  return (
    <div className="space-y-1">
      {label && (
        <label htmlFor={name} className="block text-sm font-medium text-gray-700">
          {label}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
      )}
      
      <textarea
        id={name}
        name={name}
        value={value}
        onChange={(e) => handleFieldChange(name, e.target.value)}
        onBlur={(e) => handleFieldBlur(name, e.target.value)}
        placeholder={placeholder}
        rows={rows}
        disabled={disabled}
        className={`
          w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500
          ${showError ? 'border-red-300 focus:ring-red-500' : 'border-gray-300'}
          ${disabled ? 'bg-gray-100 cursor-not-allowed' : ''}
          ${className}
        `}
      />
      
      {showError && fieldState.message && (
        <p className="text-sm text-red-600">{fieldState.message}</p>
      )}
    </div>
  );
};

// Validated select component
interface ValidatedSelectProps {
  name: string;
  options: Array<{ value: string; label: string }>;
  placeholder?: string;
  label?: string;
  required?: boolean;
  className?: string;
  disabled?: boolean;
}

export const ValidatedSelect: React.FC<ValidatedSelectProps> = ({
  name,
  options,
  placeholder,
  label,
  required = false,
  className = '',
  disabled = false
}) => {
  const {
    formData,
    validationState,
    handleFieldChange,
    handleFieldBlur
  } = useFormValidator();

  const fieldState = validationState[name] || { isValid: true, isDirty: false };
  const value = formData[name] || '';
  const showError = fieldState.isDirty && !fieldState.isValid;

  return (
    <div className="space-y-1">
      {label && (
        <label htmlFor={name} className="block text-sm font-medium text-gray-700">
          {label}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
      )}
      
      <select
        id={name}
        name={name}
        value={value}
        onChange={(e) => handleFieldChange(name, e.target.value)}
        onBlur={(e) => handleFieldBlur(name, e.target.value)}
        disabled={disabled}
        className={`
          w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500
          ${showError ? 'border-red-300 focus:ring-red-500' : 'border-gray-300'}
          ${disabled ? 'bg-gray-100 cursor-not-allowed' : ''}
          ${className}
        `}
      >
        {placeholder && (
          <option value="" disabled>
            {placeholder}
          </option>
        )}
        {options.map(option => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
      
      {showError && fieldState.message && (
        <p className="text-sm text-red-600">{fieldState.message}</p>
      )}
    </div>
  );
};

// Form submit button component
interface FormSubmitButtonProps {
  children: React.ReactNode;
  className?: string;
  disabled?: boolean;
}

export const FormSubmitButton: React.FC<FormSubmitButtonProps> = ({
  children,
  className = '',
  disabled = false
}) => {
  const { isSubmitting, isFormValid } = useFormValidator();

  return (
    <button
      type="submit"
      disabled={disabled || isSubmitting || !isFormValid}
      className={`
        btn-primary
        ${isSubmitting ? 'opacity-75 cursor-not-allowed' : ''}
        ${!isFormValid ? 'opacity-50 cursor-not-allowed' : ''}
        ${className}
      `}
    >
      {isSubmitting ? (
        <div className="flex items-center space-x-2">
          <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
          <span>Submitting...</span>
        </div>
      ) : (
        children
      )}
    </button>
  );
};

// Export common validation schemas
export { VALIDATION_SCHEMAS }; 