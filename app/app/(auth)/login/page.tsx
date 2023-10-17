'use client';
import React from 'react';
import AuthForm from '../authform';

export default function LoginPage() {

  return (
  // center the component both horizontally and vertically
    <div className='bg-gray-100 min-h-screen flex items-center justify-center'>
      <h1>Login Page</h1>
      <AuthForm val="login" /> {/* Use the AuthForm component with 'login' type */}

    </div>
  );
}
