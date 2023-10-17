'use client';
import React, { useState } from 'react';

interface MessageInputProps {
  username: string;
}

export default function MessageInput({ username }: MessageInputProps) {
  const [message, setMessage] = useState('');

  const handleSendMessage = async () => {
    if (message.trim() === '') {
      // Don't send empty messages
      return;
    }

    const messageData = {
      username: 'vericu',
      message,
      sent_at: new Date().toISOString(), // Add the sent_at field with the current UTC timestamp
    };

    try {
      const response = await fetch('http://localhost:8000/message', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include', // Include the session cookie
        body: JSON.stringify(messageData),
      });

      if (response.ok) {
        // Message sent successfully, you can handle this as needed
        console.log('Message sent successfully');
      } else {
        // Handle errors here
        console.error('Error sending message');
      }
    } catch (error) {
      console.error('Network error:', error);
    }

    // Clear the message input
    setMessage('');
  };

  return (
    <div className="message-input p-4 border rounded-lg">
      <h2 className="text-2xl mb-4">Message Input</h2>
      <div className="flex space-x-4">
        <input
          type="text"
          className="flex-1 p-2 border rounded-md text-gray-700"
          placeholder="Type your message..."
          value={message}
          onChange={(e) => setMessage(e.target.value)}
        />
        <button
          className="p-2 bg-blue-500 text-white rounded-md"
          onClick={handleSendMessage}
        >
          Send
        </button>
      </div>
    </div>
  );
}
