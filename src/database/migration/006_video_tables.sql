CREATE TABLE `video` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `date` DATE NOT NULL,
    `link` TEXT NOT NULL,
    `title` TEXT NOT NULL,
    `tags` TEXT NOT NULL,
    `duration` TEXT NOT NULL
);

CREATE UNIQUE INDEX `video_i0` ON `video` (`link`);
