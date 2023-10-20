'use client';

import React from 'react';
import MessageInput from './messageinput'; // Assuming the file path to MessageInput is correct
import Listener from './listener'; // Assuming the file path to MessageApp is correct
import {v4 as uuidv4, parse} from 'uuid';
import { MessageInputProps } from './messageinput';


export default function Page() {
  if (typeof window !== "undefined") {
    const username = localStorage.getItem('username');
    const sender_id = localStorage.getItem('sender_id');

    if (!(username && sender_id)) {
      return (
        <div className='bg-gray-100 min-h-screen flex items-center justify-center text-black'>
          <h1 className='text-4xl'>Please Login</h1>
        </div>
      );
    }

    return (
      <div className='bg-gray-100 min-h-screen items-center justify-center text-black'>
        <Listener />
        <MessageInput username={username} sender_id={sender_id} />
      </div>
    );
  } else {
    return (
      <div className='bg-gray-100 min-h-screen flex items-center justify-center text-black'>
        <h1 className='text-4xl'>Please Login</h1>
      </div>
    );
  }
}


