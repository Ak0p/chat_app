import React from 'react';
import AuthForm from '../authform';

export default function RegisterPage() {
  return (
    <div>
      <h1>Register Page</h1>
      <AuthForm val="register" /> {/* Use the AuthForm component with 'register' type */}
    </div>
  );
}
