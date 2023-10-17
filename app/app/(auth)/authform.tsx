'use client';
import React, { useState } from 'react';
import Link from 'next/link';
// import { redirect } from 'next/navigation';
import { useRouter } from 'next/router';
interface AuthFormProps {
  val: 'login' | 'register';
}

const AuthForm: React.FC<AuthFormProps> = ({ val }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

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
      // console.log('in try, ',val, data);
      console.log("vvvvv", JSON.stringify(data));
      // Implement your API call here based on the 'type' (register or login)
      // You can use the 'type' prop to determine which API endpoint to call.
      // Example: if (type === 'register') make a registration API call
      // Example: if (type === 'login') make a login API call

      // After the API call, you can handle the response accordingly
      // For example, display a success message or error message.
      if (val === 'register') {
        // console.log('Registering user with:', data);
        // send the data in a POST request to localhost:8000/register
        const response = await fetch('http://localhost:8000/register', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(data),
        });

        // parse the response
        // const parsedResponse = await response.json();
        // // check if the response is successful
        // // if successful, redirect /login
        // // if unsuccessful, display an error message and reload component
        // // (you can display an error message by setting a state variable)
        // if (parsedResponse.success) {
        //   window.location.href = '/login';
        // }
        // else {
        //   alert(parsedResponse.message);
        //   window.location.reload();
        // }

      } else if (val === 'login') {
        // console.log('Logging in user with:', data);
        // send the data in a POST request to localhost:8000/login
        const response = await fetch('http://localhost:8000/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify(data),
        });


         
        // check if the response is successful
        // if successful, redirect /chat
        // if (response.status === 200) {
        //  // return redirect("/chat");
        //  const router = useRouter();
        //  router.push('/chat');
        // }
        // // if unsuccessful, display an error message and reload component
        // // (you can display an error message by setting a state variable)
        // else {
        //   alert(response.statusText);
        //   window.location.reload();
        // }
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
