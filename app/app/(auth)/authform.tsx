'use client';
import React, { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
interface AuthFormProps {
  val: 'login' | 'register';
}

const AuthForm: React.FC<AuthFormProps> = ({ val }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const router = useRouter();

  const handleAuth = async () => {
    console.log('Authenticating user...');
    // Create a data object to send to the server
    // the object should have fields 'username' and 'password'
    const data = {
      username,
      password,
    };

    console.log('Sending data:', data);

    try {
      if (val === 'register') {
        const response = await fetch('http://localhost:8000/register', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(data),
        });

        if (response.ok) {
          router.push('/login')
        } else {
          alert(response.statusText);
        }

      } else if (val === 'login') {
        const response = await fetch('http://localhost:8000/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify(data),
        });

        if (response.ok) {
          try {
          console.log('response:', response);
          const sender_id = await response.text();
          localStorage.setItem('sender_id', sender_id);
          localStorage.setItem('username', username); 
          } catch (error) {
            console.log('Jsonu matii:', error);
          }
          router.push('/chat')
        } else {
          alert(response.statusText);
        }
    
      } else {
        console.log('Invalid type:', val);
      }
    } catch (error) {
      console.error('Error:', error);
      // Handle error, e.g., display an error message to the user
    }
  };

   return (
    <div className="w-64 mx-auto bg-white p-4 text-black rounded-lg shadow-md">
      <h2 className="text-lg font-bold text-center mb-2">{val === 'register' ? 'Register' : 'Login'}</h2>
      <form>
        <div className="mb-4">
          <label htmlFor={`${val}-username`} className="block text-gray-700 font-bold">Username:</label>
          <input
            type="text"
            id={`${val}-username`}
            value={username}
            onChange={(e) => { 
              console.log('e.target.value:', e.target.value);
              setUsername(e.target.value)}}
            className="w-full px-3 py-2 border rounded text-black"
            required
          />
        </div>
        <div className="mb-4">
          <label htmlFor={`${val}-password`} className="block text-gray-700 font-bold">Password:</label>
          <input
            type="password"
            id={`${val}-password`}
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full px-3 py-2 border rounded text-black"
            required
          />
        </div>
        <button
          type="button"
          onClick={handleAuth}
          className="w-full bg-blue-500 text-white font-bold py-2 rounded hover:bg-blue-600 cursor-pointer"
        >
          {val === 'register' ? 'Register' : 'Login'}
        </button>
      </form>
    </div>
  );};

export default AuthForm;
