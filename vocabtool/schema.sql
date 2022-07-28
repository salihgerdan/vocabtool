CREATE TABLE IF NOT EXISTS texts (
    id          INTEGER PRIMARY KEY NOT NULL AUTO_INCREMENT,
    text        TEXT NOT NULL,
    tokenized   TEXT NOT NULL,
    lang        VARCHAR(5) NOT NULL DEFAULT "zh_CN",
    user        INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS words (
    id          INTEGER PRIMARY KEY NOT NULL AUTO_INCREMENT,
    word        TEXT NOT NULL,
    state       INTEGER NOT NULL DEFAULT 0,
    notes       TEXT NOT NULL DEFAULT "",
    lang        VARCHAR(5) NOT NULL DEFAULT "zh_CN",
    user        INTEGER NOT NULL,
    CONSTRAINT userword UNIQUE NONCLUSTERED (user, word)
);
