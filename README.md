# Nuclear Nightmare web API


This repository (hopefully) contains the source code to the Nuclear Nightmare Web API.

**Why?**\
Well, since the game plans to add Mod support, that is basically opening the door to cheaters! Bad!\
If the points and other future additions get moved to the server, then there will be no problems!

## API Routes and Responses

***User Creation*** POST (`/login/<id>`)\
This route can respond with a few things:
- No Content (The user isnt real on steam. AKA an invalid Steam ID) **204**
- Internal Server Error (Failed to write to the DB) **500**
- User data (**Defined below**)

***User Data*** GET (`/data/<id>`)\
This route is pretty simple:
- No Content (The user hasnt been registered yet, or its an invalid Steam ID) **204**
- Internal Server Error (DB is broken) **500**
- User Data (**Defined below**)

***Update Points*** POST (`/data/<id>/update_points/<(+|-)points>`)\
This route is also simple:
- No Content (The user hasnt been registered yet, or its an invalid Steam ID) **204**
- Internal Server Error (DB is broken) **500**
- Ok (Updated) **200**

# User Data
The good stuff, here is the actual JSON response.
```json
{
	"steam_id": "76561198802528998",
	"last_known_username": "Infinity Dev",
	"points": 1040,
	"perks": {
		"hardcore_survivor": false,
		"doggy_treats": false,
		"immune_system": false,
		"strong_scent": false,
		"infection_detector": false,
		"gas_guzzler": false,
		"drivers_license": false,
		"home_free": false,
		"gen_jockey": false,
		"oil_conservation": false,
		"soft_feet": false,
		"monster_hunter": false,
		"flare_signal": false,
		"acrobatic_instructor": false,
		"pain_medication": false,
		"military_veteran": false,
		"hungry_humans": false,
		"vaccinated_scientist": false,
		"emergency_fund": false,
		"warn_blooded": false,
		"speed_demon": false,
		"pilot_license": false,
		"rechargeable_battery": false,
		"nuclear_awareness": false,
		"elusive_fighter": false,
		"dog_whisper": false,
		"hide_n_seek": false,
		"fire_resistance": false,
		"classified_collector": false,
		"self_defense": false,
		"cold_blooded": false,
		"heated_jacket": false,
		"health_regeneration": false,
		"camp_counseler": false,
		"nightmare_detector": false,
		"gps_navigator": false,
		"rocket_booster": false,
		"winter_mystery": false,
		"jetpack_ride": false,
		"night_vision": false
	}
}
```