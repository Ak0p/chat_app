'use client';
import React, { useState } from 'react'




import Link from 'next/link';

export default function Home() {
  return (
    <div className="bg-gray-100 min-h-screen flex items-center justify-center">
      <div className="bg-white p-8 rounded-lg shadow-md w-96">
        <h1 className="text-2xl font-bold mb-4">Register/Login Menu</h1>

        <div className="mb-4 justify-center">
          <h2 className="text-lg  font-bold mb-2">Register</h2>
          <Link href="/register" className="font-bold text-center block bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 cursor-pointer">
            Register
          </Link>
        </div>

        <div className="mb-4  justify-center">
          <h2 className="text-lg font-bold mb-2">Login</h2>
          <Link href="/login" className="font-bold text-center block bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 cursor-pointer">
            Login
          </Link>
        </div>
      </div>
    </div>
  );
}



