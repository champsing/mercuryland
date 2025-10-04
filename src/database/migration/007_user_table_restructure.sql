
-- 1. Create the new `user` table with the desired schema
CREATE TABLE `user` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `youtube` TEXT UNIQUE NOT NULL,
    `discord` BIGINT UNIQUE,
    `updated_at` DATETIME NOT NULL,
    `display` TEXT NOT NULL,
    `coin` NUMBER NOT NULL
);

-- 2. Insert transformed data from the old `coin` table
INSERT INTO `user` (`youtube`, `discord`, `updated_at`, `display`, `coin`)
SELECT 
    `id` AS youtube,
    NULLIF(`discord_id`, 0) AS discord,
    `updated_at`,
    `display`,
    `coin`
FROM `coin`;

-- 3. Drop the old `coin` table
DROP TABLE `coin`;
