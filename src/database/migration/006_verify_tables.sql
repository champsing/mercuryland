CREATE TABLE `verify` (
    `verification_code` TEXT NOT NULL,
    `yt_ch_id` TEXT NOT NULL,
    `generated_at` DATETIME NOT NULL
);

CREATE INDEX `verify_i0` ON `verify` (`generated_at`);