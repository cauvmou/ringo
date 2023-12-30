BEGIN;
DROP TABLE IF EXISTS field;
DROP TABLE IF EXISTS board;
DROP TABLE IF EXISTS player;
DROP TABLE IF EXISTS room;
DROP TABLE IF EXISTS signing_key;

PRAGMA foreign_keys = ON;

CREATE TABLE signing_key
(
    pk_signing_key VARCHAR(256) NOT NULL,
    PRIMARY KEY (pk_signing_key)
);

CREATE TABLE room
(
    pk_room_code    VARCHAR(6)    NOT NULL,
    board_size      INTEGER       NOT NULL,
    list            TEXT          NOT NULL,
    duration        DECIMAL(8, 2) NOT NULL,
    started         DATE,
    generation_type VARCHAR(3)    NOT NULL,
    PRIMARY KEY (pk_room_code)
);

CREATE TABLE player
(
    pk_username     VARCHAR(16) NOT NULL,
    pk_fk_room_code VARCHAR(6)  NOT NULL,
    cosmetic1       VARCHAR(32) NOT NULL,
    cosmetic2       VARCHAR(32) NOT NULL,
    cosmetic3       VARCHAR(32) NOT NULL,
    PRIMARY KEY (pk_username, pk_fk_room_code),
    FOREIGN KEY (pk_fk_room_code) REFERENCES room (pk_room_code)
);

CREATE TABLE board
(
    pk_fk_username  VARCHAR(16) NOT NULL,
    pk_fk_room_code VARCHAR(6)  NOT NULL,
    seed_position   INTEGER     NOT NULL,
    seed_fields     INTEGER     NOT NULL,
    PRIMARY KEY (pk_fk_username, pk_fk_room_code),
    FOREIGN KEY (pk_fk_username) REFERENCES player (pk_username),
    FOREIGN KEY (pk_fk_room_code) REFERENCES player (pk_fk_room_code)
);

CREATE TABLE field
(
    pk_fk_username  VARCHAR(16) NOT NULL,
    pk_fk_room_code VARCHAR(6)  NOT NULL,
    pk_index        INTEGER     NOT NULL,
    seed_position   INTEGER     NOT NULL,
    seed_fields     INTEGER     NOT NULL,
    PRIMARY KEY (pk_fk_username, pk_fk_room_code, pk_index),
    FOREIGN KEY (pk_fk_username) REFERENCES board (pk_fk_username),
    FOREIGN KEY (pk_fk_room_code) REFERENCES board (pk_fk_room_code)
);

ALTER TABLE room
    ADD COLUMN fk_host_username VARCHAR(16) REFERENCES player (pk_username);
COMMIT;