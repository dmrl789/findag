import React, { useState, useEffect } from 'react';
import { Shield, CheckCircle, AlertCircle, Loader2, QrCode, Copy } from 'lucide-react';
import { finDAGApi } from '../../services/api';

interface TwoFactorSetupProps {
  onSetupSuccess?: (message: string) => void;
  onSetupError?: (error: string) => void;
  onCancel?: () => void;
}

export const TwoFactorSetup: React.FC<TwoFactorSetupProps> = ({ 
  onSetupSuccess, 
  onSetupError,
  onCancel 
}) => {
  const [step, setStep] = useState<'setup' | 'verify'>('setup');
  const [secret, setSecret] = useState('');
  const [qrUrl, setQrUrl] = useState('');
  const [verificationCode, setVerificationCode] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  useEffect(() => {
    // Initialize 2FA setup when component mounts
    initializeSetup();
  }, []);

  const initializeSetup = async () => {
    setLoading(true);
    setError(null);

    try {
      const response = await finDAGApi.setup2FA();
      
      if (response.secret && response.qr_url) {
        setSecret(response.secret);
        setQrUrl(response.qr_url);
        setStep('verify');
      } else {
        setError('Failed to initialize 2FA setup');
      }
    } catch (error: any) {
      const errorMessage = error.message || 'Failed to initialize 2FA setup';
      setError(errorMessage);
      
      if (onSetupError) {
        onSetupError(errorMessage);
      }
    } finally {
      setLoading(false);
    }
  };

  const handleVerificationCodeChange = (value: string) => {
    setVerificationCode(value);
    // Clear error when user starts typing
    if (error) setError(null);
  };

  const handleVerifyCode = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!verificationCode || verificationCode.length !== 6) {
      setError('Please enter a valid 6-digit code');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const response = await finDAGApi.enable2FA(secret, verificationCode);
      
      if (response.success) {
        setSuccess(response.message || 'Two-factor authentication enabled successfully!');
        
        if (onSetupSuccess) {
          onSetupSuccess(response.message || 'Two-factor authentication enabled successfully!');
        }
      } else {
        setError(response.message || 'Failed to enable 2FA. Please try again.');
        
        if (onSetupError) {
          onSetupError(response.message || 'Failed to enable 2FA.');
        }
      }
    } catch (error: any) {
      const errorMessage = error.message || 'Failed to enable 2FA. Please try again.';
      setError(errorMessage);
      
      if (onSetupError) {
        onSetupError(errorMessage);
      }
    } finally {
      setLoading(false);
    }
  };

  const copySecretToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(secret);
      // You could show a toast notification here
    } catch (error) {
      console.error('Failed to copy secret to clipboard:', error);
    }
  };

  if (loading && step === 'setup') {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
        <div className="max-w-md w-full space-y-8">
          <div className="text-center">
            <Loader2 className="mx-auto h-12 w-12 animate-spin text-primary-600" />
            <h2 className="mt-6 text-xl font-semibold text-gray-900">
              Setting up 2FA...
            </h2>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div>
          <div className="mx-auto h-12 w-12 bg-primary-600 rounded-lg flex items-center justify-center">
            <Shield className="h-6 w-6 text-white" />
          </div>
          <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
            Set up Two-Factor Authentication
          </h2>
          <p className="mt-2 text-center text-sm text-gray-600">
            Add an extra layer of security to your account
          </p>
        </div>

        {step === 'verify' && (
          <div className="space-y-6">
            {/* QR Code */}
            <div className="text-center">
              <h3 className="text-lg font-medium text-gray-900 mb-4">
                Scan the QR code
              </h3>
              <div className="bg-white p-4 rounded-lg border border-gray-200 inline-block">
                {qrUrl ? (
                  <img 
                    src={qrUrl} 
                    alt="2FA QR Code" 
                    className="w-48 h-48"
                  />
                ) : (
                  <div className="w-48 h-48 bg-gray-100 flex items-center justify-center">
                    <QrCode className="w-12 h-12 text-gray-400" />
                  </div>
                )}
              </div>
            </div>

            {/* Secret Key */}
            <div>
              <h3 className="text-lg font-medium text-gray-900 mb-2">
                Or enter the secret key manually
              </h3>
              <div className="flex items-center space-x-2">
                <input
                  type="text"
                  value={secret}
                  readOnly
                  className="flex-1 px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-sm font-mono"
                />
                <button
                  type="button"
                  onClick={copySecretToClipboard}
                  className="p-2 text-gray-400 hover:text-gray-600"
                  title="Copy to clipboard"
                >
                  <Copy className="w-4 h-4" />
                </button>
              </div>
              <p className="mt-1 text-xs text-gray-500">
                Enter this code in your authenticator app
              </p>
            </div>

            {/* Verification Form */}
            <form onSubmit={handleVerifyCode} className="space-y-4">
              <div>
                <label htmlFor="verificationCode" className="block text-sm font-medium text-gray-700">
                  Verification Code
                </label>
                <input
                  id="verificationCode"
                  type="text"
                  value={verificationCode}
                  onChange={(e) => handleVerificationCodeChange(e.target.value)}
                  placeholder="Enter 6-digit code"
                  maxLength={6}
                  className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
                  disabled={loading}
                />
              </div>

              {/* Error Display */}
              {error && (
                <div className="rounded-md bg-danger-50 p-4">
                  <div className="flex">
                    <div className="flex-shrink-0">
                      <AlertCircle className="h-5 w-5 text-danger-400" />
                    </div>
                    <div className="ml-3">
                      <h3 className="text-sm font-medium text-danger-800">
                        {error}
                      </h3>
                    </div>
                  </div>
                </div>
              )}

              {/* Success Display */}
              {success && (
                <div className="rounded-md bg-success-50 p-4">
                  <div className="flex">
                    <div className="flex-shrink-0">
                      <CheckCircle className="h-5 w-5 text-success-400" />
                    </div>
                    <div className="ml-3">
                      <h3 className="text-sm font-medium text-success-800">
                        {success}
                      </h3>
                    </div>
                  </div>
                </div>
              )}

              {/* Action Buttons */}
              <div className="flex space-x-3">
                <button
                  type="submit"
                  disabled={loading}
                  className="flex-1 bg-primary-600 text-white px-4 py-2 rounded-md hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50"
                >
                  {loading ? (
                    <>
                      <Loader2 className="animate-spin -ml-1 mr-2 h-4 w-4" />
                      Verifying...
                    </>
                  ) : (
                    'Verify and Enable'
                  )}
                </button>
                
                {onCancel && (
                  <button
                    type="button"
                    onClick={onCancel}
                    className="flex-1 bg-gray-300 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                  >
                    Cancel
                  </button>
                )}
              </div>
            </form>

            {/* Instructions */}
            <div className="bg-blue-50 p-4 rounded-md">
              <h4 className="text-sm font-medium text-blue-900 mb-2">
                How to set up 2FA:
              </h4>
              <ol className="text-sm text-blue-800 space-y-1">
                <li>1. Install an authenticator app (Google Authenticator, Authy, etc.)</li>
                <li>2. Scan the QR code or manually enter the secret key</li>
                <li>3. Enter the 6-digit code from your app</li>
                <li>4. Click "Verify and Enable" to complete setup</li>
              </ol>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}; 