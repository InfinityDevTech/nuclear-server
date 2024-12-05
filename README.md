# Nuclear Nightmare web API


This repository (hopefully) contains the source code to the Nuclear Nightmare Web API.

**Why?**\
Well, since the game plans to add Mod support, that is basically opening the door to cheaters! Bad!\
If the points and other future additions get moved to the server, then there will be no problems!

## API Routes and Responses

***User Creation*** (`/login/<id>`)\
This route can respond with a few things:
- No Content (The user isnt real on steam. AKA an invalid Steam ID) **204**
- Internal Server Error (Failed to write to the DB) **500**
- User data (**Defined below**)

***User Data*** (`/data/<id>`)\
This route is pretty simple:
- No Content (The user hasnt been registered yet, or its an invalid Steam ID) **204**
- Internal Server Error (DB is broken) **500**
- User Data (**Defined below**)

***Update Points*** (`/data/<id>/update_points/<(+|-)points>`)\
This route is also simple:
- No Content (The user hasnt been registered yet, or its an invalid Steam ID) **204**
- Internal Server Error (DB is broken) **500**
- Ok (Updated) **200**