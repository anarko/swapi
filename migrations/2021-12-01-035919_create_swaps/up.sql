-- Your SQL goes here
CREATE TABLE swaps (
  swap_uuid VARCHAR NOT NULL PRIMARY KEY,
  pair VARCHAR NOT NULL,
  side VARCHAR NOT NULL,
  book VARCHAR NOT NULL,
  quantity FLOAT NOT NULL,
  time_satmp FLOAT NOT NULL,
  fee FLOAT NOT NULL,
)

