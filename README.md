# Chat app

This is a working demo of a simple realtime chat app with a web interface.

## How does it work?

The user has to register an account and log in in order to use the app.  
On the chat page is simple interface which displays a prompt and the message history.

## Scope  
The scope of this project was learning to work with REST API's, async programming and databases.  
The frontent was completely outside of the project's scope and thus I did not pay much attention to it.  

## How to run

- In order to run the project you will need a working postgres containter in docker.  
- Complete the **DATABASE_URL** env variable in the /webapi folder with the url from postgres.    
- After starting the database start the backend using `cargo run` in the webapi directory.  
- Then start the frontend by running `npm run dev` in the /app/app folder.  

## Misc  

For the app to run you **need** to set the **SECRET_KEY** env variable in the same file as the **DATABASE_URL** variable.  
Generate a new key using openssl. 
