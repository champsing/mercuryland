CREATE TABLE `wheel` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `secret` TEXT NOT NULL,
    `content` TEXT NOT NULL,
    `updated_at` DATETIME NOT NULL
);

CREATE INDEX `wheel_i0` ON `wheel` (`updated_at`);

