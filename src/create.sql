CREATE TABLE room
(
    pk_room_code     VARCHAR(6) PRIMARY KEY,
    board_size       INTEGER             NOT NULL,
    list             VARCHAR(4294967296) NOT NULL,
    duration         DECIMAL(8, 2)       NOT NULL,
    started          DATE,
    generation_type  VARCHAR(3)          NOT NULL,
    fk_host_username VARCHAR(16)         NOT NULL
);

CREATE TABLE player
(
    pk_username     VARCHAR(16) PRIMARY KEY,
    pk_fk_room_code VARCHAR(6) PRIMARY KEY,
    cosmetic1       VARCHAR(32) NOT NULL,
    cosmetic2       VARCHAR(32) NOT NULL,
    cosmetic3       VARCHAR(32) NOT NULL,
    FOREIGN KEY (pk_fk_room_code) REFERENCES room (pk_room_code)
);

CREATE TABLE board
(
    pk_fk_username  VARCHAR(16) PRIMARY KEY,
    pk_fk_room_code VARCHAR(6) PRIMARY KEY,
    seed_position   INTEGER NOT NULL,
    seed_fields     INTEGER NOT NULL,
    FOREIGN KEY (pk_fk_username) REFERENCES player (pk_username),
    FOREIGN KEY (pk_fk_room_code) REFERENCES player (pk_fk_room_code)
);

CREATE TABLE board
(
    pk_fk_username  VARCHAR(16) PRIMARY KEY,
    pk_fk_room_code VARCHAR(6) PRIMARY KEY,
    pk_index        INTEGER PRIMARY KEY,
    seed_position   INTEGER NOT NULL,
    seed_fields     INTEGER NOT NULL,
    FOREIGN KEY (pk_fk_username) REFERENCES board (pk_username),
    FOREIGN KEY (pk_fk_room_code) REFERENCES board (pk_fk_room_code)
);
);

ALTER TABLE room
    ADD FOREIGN KEY (fk_host_username) REFERENCES player (pk_username);