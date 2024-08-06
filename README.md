# JamClub in Rust

This is my first projet in Rust. I'm an hobbiist developper.
i
## JamClub modules
There are three modules :

- jamclub_seaorm which implements database models and model_controller.

- jamclub_csvimport which implements data CSV import to the database. It uses jamclub_seaorm.

- discord-cmd-slash : a simple discord bot that provides slash commands that sometimes read database using jamclub_seaorm to provide information replied to the end user using ephemeral embeds.

## seaorm 

SeaOrm provides a convenient method to generate entities from SQL Tables directly by reading the Database.

## serde

Serde provides an easy way to deserialize content making some verifications on types including database enums and array of database enums (using postgres).

## serenity and poise

I'm using this framework to implement the slash commands.

## python ...

The csv files are downloaded using a python script (could not be done easily in RUST because I was missing a convenient library). These csv files have to be processed by RUST which is much faster.


