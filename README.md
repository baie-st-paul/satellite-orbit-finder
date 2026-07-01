# satelite-orbit-finder
KSP inspired

## Instructions

- Go to https://www.space-track.org/
- create account (save password and username)
- have a postgress database (I recommend using docker)
- install DieselCLI (refer to diesel doc for instalation)
- create .env file in project root folder
- type :
    ````
    identity=(username)
    password=(password)
    DATABASE_URL=(database url, format: postgres://username:password@localhost/sat)
    ```` 


## Sources

- Diesel docs: https://diesel.rs/guides/getting-started.html