CREATE TABLE `image` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `name` TEXT NOT NULL UNIQUE,
    `data` BLOB NOT NULL,
    `mime` TEXT NOT NULL
);