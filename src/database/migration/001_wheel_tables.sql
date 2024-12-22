CREATE TABLE IF NOT EXISTS `wheel` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `secret` TEXT NOT NULL,
    `content` TEXT NOT NULL,
    `updated_at` DATETIME NOT NULL
);

CREATE INDEX IF NOT EXISTS `wheel_i0` ON `wheel` (`updated_at`);

