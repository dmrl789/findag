// Security utilities for input validation, CSRF protection, and data sanitization

// Input validation schemas
export interface ValidationRule {
  required?: boolean;
  minLength?: number;
  maxLength?: number;
  pattern?: RegExp;
  type?: 'email' | 'url' | 'number' | 'integer' | 'date' | 'phone';
  custom?: (value: any) => boolean;
  message?: string;
}

export interface ValidationSchema {
  [key: string]: ValidationRule;
}

// Common validation patterns
export const VALIDATION_PATTERNS = {
  email: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  url: /^https?:\/\/.+/,
  phone: /^\+?[\d\s\-\(\)]+$/,
  username: /^[a-zA-Z0-9_]{3,20}$/,
  password: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/,
  walletAddress: /^[0-9a-fA-F]{40}$/,
  transactionHash: /^[0-9a-fA-F]{64}$/,
  amount: /^\d+(\.\d{1,8})?$/,
  positiveNumber: /^[1-9]\d*$/,
  alphanumeric: /^[a-zA-Z0-9]+$/,
  noSpecialChars: /^[a-zA-Z0-9\s]+$/
};

// Validation messages
export const VALIDATION_MESSAGES = {
  required: 'This field is required.',
  email: 'Please enter a valid email address.',
  url: 'Please enter a valid URL.',
  phone: 'Please enter a valid phone number.',
  username: 'Username must be 3-20 characters long and contain only letters, numbers, and underscores.',
  password: 'Password must be at least 8 characters long and contain uppercase, lowercase, number, and special character.',
  walletAddress: 'Please enter a valid wallet address.',
  transactionHash: 'Please enter a valid transaction hash.',
  amount: 'Please enter a valid amount.',
  positiveNumber: 'Please enter a positive number.',
  minLength: (min: number) => `Must be at least ${min} characters long.`,
  maxLength: (max: number) => `Must be no more than ${max} characters long.`,
  pattern: 'Please enter a valid format.',
  custom: 'Please check your input.'
};

// Input validation class
export class InputValidator {
  private static instance: InputValidator;

  static getInstance(): InputValidator {
    if (!InputValidator.instance) {
      InputValidator.instance = new InputValidator();
    }
    return InputValidator.instance;
  }

  // Validate a single field
  validateField(value: any, rule: ValidationRule): { isValid: boolean; message?: string } {
    // Check if required
    if (rule.required && (!value || value.toString().trim() === '')) {
      return { isValid: false, message: rule.message || VALIDATION_MESSAGES.required };
    }

    // Skip validation if value is empty and not required
    if (!value && !rule.required) {
      return { isValid: true };
    }

    const stringValue = value?.toString() || '';

    // Check length constraints
    if (rule.minLength && stringValue.length < rule.minLength) {
      return { isValid: false, message: rule.message || VALIDATION_MESSAGES.minLength(rule.minLength) };
    }

    if (rule.maxLength && stringValue.length > rule.maxLength) {
      return { isValid: false, message: rule.message || VALIDATION_MESSAGES.maxLength(rule.maxLength) };
    }

    // Check pattern
    if (rule.pattern && !rule.pattern.test(stringValue)) {
      return { isValid: false, message: rule.message || VALIDATION_MESSAGES.pattern };
    }

    // Check type-specific validation
    if (rule.type) {
      const typeValidation = this.validateType(stringValue, rule.type);
      if (!typeValidation.isValid) {
        return typeValidation;
      }
    }

    // Check custom validation
    if (rule.custom && !rule.custom(value)) {
      return { isValid: false, message: rule.message || VALIDATION_MESSAGES.custom };
    }

    return { isValid: true };
  }

  // Validate multiple fields
  validateForm(data: Record<string, any>, schema: ValidationSchema): {
    isValid: boolean;
    errors: Record<string, string>;
  } {
    const errors: Record<string, string> = {};
    let isValid = true;

    for (const [field, rule] of Object.entries(schema)) {
      const validation = this.validateField(data[field], rule);
      if (!validation.isValid) {
        errors[field] = validation.message!;
        isValid = false;
      }
    }

    return { isValid, errors };
  }

  // Type-specific validation
  private validateType(value: string, type: string): { isValid: boolean; message?: string } {
    switch (type) {
      case 'email':
        return VALIDATION_PATTERNS.email.test(value)
          ? { isValid: true }
          : { isValid: false, message: VALIDATION_MESSAGES.email };

      case 'url':
        return VALIDATION_PATTERNS.url.test(value)
          ? { isValid: true }
          : { isValid: false, message: VALIDATION_MESSAGES.url };

      case 'phone':
        return VALIDATION_PATTERNS.phone.test(value)
          ? { isValid: true }
          : { isValid: false, message: VALIDATION_MESSAGES.phone };

      case 'number':
        return !isNaN(Number(value))
          ? { isValid: true }
          : { isValid: false, message: 'Please enter a valid number.' };

      case 'integer':
        return Number.isInteger(Number(value))
          ? { isValid: true }
          : { isValid: false, message: 'Please enter a valid integer.' };

      case 'date':
        const date = new Date(value);
        return !isNaN(date.getTime())
          ? { isValid: true }
          : { isValid: false, message: 'Please enter a valid date.' };

      default:
        return { isValid: true };
    }
  }

  // Sanitize input
  sanitizeInput(input: string): string {
    return input
      .trim()
      .replace(/[<>]/g, '') // Remove potential HTML tags
      .replace(/javascript:/gi, '') // Remove javascript: protocol
      .replace(/on\w+=/gi, '') // Remove event handlers
      .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, ''); // Remove script tags
  }

  // Sanitize object
  sanitizeObject(obj: Record<string, any>): Record<string, any> {
    const sanitized: Record<string, any> = {};
    
    for (const [key, value] of Object.entries(obj)) {
      if (typeof value === 'string') {
        sanitized[key] = this.sanitizeInput(value);
      } else if (typeof value === 'object' && value !== null) {
        sanitized[key] = this.sanitizeObject(value);
      } else {
        sanitized[key] = value;
      }
    }

    return sanitized;
  }
}

// CSRF Protection
export class CSRFProtection {
  private static token: string | null = null;

  // Generate CSRF token
  static generateToken(): string {
    const token = Math.random().toString(36).substring(2) + Date.now().toString(36);
    this.token = token;
    return token;
  }

  // Get current token
  static getToken(): string | null {
    return this.token;
  }

  // Validate token
  static validateToken(token: string): boolean {
    return token === this.token;
  }

  // Add token to headers
  static addTokenToHeaders(headers: Record<string, string>): Record<string, string> {
    if (this.token) {
      headers['X-CSRF-Token'] = this.token;
    }
    return headers;
  }
}

// Security Headers
export class SecurityHeaders {
  // Get security headers for API requests
  static getSecurityHeaders(): Record<string, string> {
    return {
      'X-Content-Type-Options': 'nosniff',
      'X-Frame-Options': 'DENY',
      'X-XSS-Protection': '1; mode=block',
      'Referrer-Policy': 'strict-origin-when-cross-origin',
      'Content-Security-Policy': "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';"
    };
  }

  // Add security headers to fetch request
  static addSecurityHeaders(request: Request): Request {
    const headers = new Headers(request.headers);
    
    Object.entries(this.getSecurityHeaders()).forEach(([key, value]) => {
      headers.set(key, value);
    });

    return new Request(request, { headers });
  }
}

// Rate Limiting
export class RateLimiter {
  private static requests: Map<string, { count: number; resetTime: number }> = new Map();
  private static maxRequests = 100; // Max requests per window
  private static windowMs = 60000; // 1 minute window

  // Check if request is allowed
  static isAllowed(key: string): boolean {
    const now = Date.now();
    const request = this.requests.get(key);

    if (!request || now > request.resetTime) {
      // Reset or create new request tracking
      this.requests.set(key, { count: 1, resetTime: now + this.windowMs });
      return true;
    }

    if (request.count >= this.maxRequests) {
      return false;
    }

    request.count++;
    return true;
  }

  // Get remaining requests
  static getRemainingRequests(key: string): number {
    const request = this.requests.get(key);
    if (!request) return this.maxRequests;
    return Math.max(0, this.maxRequests - request.count);
  }

  // Get reset time
  static getResetTime(key: string): number {
    const request = this.requests.get(key);
    return request?.resetTime || Date.now() + this.windowMs;
  }

  // Clear expired entries
  static cleanup(): void {
    const now = Date.now();
    for (const [key, request] of this.requests.entries()) {
      if (now > request.resetTime) {
        this.requests.delete(key);
      }
    }
  }
}

// Data encryption utilities
export class DataEncryption {
  // Simple base64 encoding (for non-sensitive data)
  static encode(data: string): string {
    return btoa(encodeURIComponent(data));
  }

  // Simple base64 decoding
  static decode(encoded: string): string {
    return decodeURIComponent(atob(encoded));
  }

  // Hash sensitive data (for comparison only)
  static async hash(data: string): Promise<string> {
    const encoder = new TextEncoder();
    const dataBuffer = encoder.encode(data);
    const hashBuffer = await crypto.subtle.digest('SHA-256', dataBuffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  }
}

// Export singleton instances
export const inputValidator = InputValidator.getInstance();

// Common validation schemas
export const VALIDATION_SCHEMAS = {
  login: {
    username: { required: true, minLength: 3, maxLength: 20 },
    password: { required: true, minLength: 8 }
  },
  register: {
    username: { required: true, pattern: VALIDATION_PATTERNS.username },
    email: { required: true, type: 'email' },
    password: { required: true, pattern: VALIDATION_PATTERNS.password },
    confirmPassword: { required: true }
  },
  trading: {
    symbol: { required: true, pattern: /^[A-Z]{3}\/[A-Z]{3}$/ },
    quantity: { required: true, type: 'number' },
    price: { required: true, type: 'number' }
  },
  wallet: {
    address: { required: true, pattern: VALIDATION_PATTERNS.walletAddress },
    amount: { required: true, pattern: VALIDATION_PATTERNS.amount }
  }
}; 