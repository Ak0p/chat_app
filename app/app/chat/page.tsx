import React from 'react';
import MessageInput from './messageinput'; // Assuming the file path to MessageInput is correct
import Listener from './listener'; // Assuming the file path to MessageApp is correct

interface Props {
  username: string;
}

export default function Page(props: Props) {
  return (
    <div>
      <h1>Message App</h1>
      <MessageInput username={props.username} />
      <Listener />
    </div>
  );
}

