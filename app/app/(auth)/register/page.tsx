import React from 'react';
import AuthForm from '../authform';

export default function RegisterPage() {
  return (
    <div className='bg-gray-100 min-h-screen flex items-center justify-center'>
      <AuthForm val="register" /> {/* Use the AuthForm component with 'register' type */}
    </div>
  );
}
