CREATE TABLE `user` (
    `id` TEXT PRIMARY KEY NOT NULL,
    `coin` NUMBER NOT NULL,
    `updated_at` DATETIME NOT NULL
);

CREATE INDEX `user_i2` ON `user` (`updated_at`);

