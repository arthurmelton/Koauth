# Koauth

Koauth is an authorization client for `Knockout City`. It uses the `secret` parameter in the client and servers to act like a user's password instead of the same password being shared between all users. I don't know the hashing algorithm used, but all the hashed passwords are stored in the database table `passwords` so you can use any program you want to edit these values (like if you want to have a website for users to sign up).

This automatically allows a user to join the server and create an account. If you attempt to connect and the username is not taken, it will save whatever password you were using into the database.

There is also no way to change your password; the only way would be for you to find out what the hash of your new password is and ask the server admin to change your password to the new hash.
