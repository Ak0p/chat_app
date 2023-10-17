'use client';
import React, { useEffect, useState } from 'react';

interface Message {
  message: string;
  username: string;
  sent_at: string;
}

export default function Listener() {
  const [messages, setMessages] = useState<Message[]>([]);

  useEffect(() => {
    const eventSource = new EventSource('http://localhost:8000/events');

    eventSource.onmessage = (event) => {
      const eventData = JSON.parse(event.data);

      if (eventData.message && eventData.username && eventData.sent_at) {
        const newMessage: Message = {
          message: eventData.message,
          username: eventData.username,
          sent_at: eventData.sent_at,
        };

        setMessages((prevMessages) => [...prevMessages, newMessage]);
      }
    };

    eventSource.onerror = (error) => {
      console.error('EventSource failed:', error);
      eventSource.close();
    };

    return () => {
      eventSource.close();
    };
  }, []);

  return (
    <div>
      <h1>Message App</h1>
      <div className="message-container">
        {messages.map((message, index) => (
          <div className="message" key={index}>
            <div className="username">Username: {message.username}</div>
            <div className="sent-at">Sent At: {message.sent_at}</div>
            <div className="text">Message: {message.message}</div>
          </div>
        ))}
      </div>
    </div>
  );
}

