ALTER TABLE `user`
RENAME TO `coin`;

DROP INDEX `user_i2`;

CREATE INDEX `coin_i0` ON `coin` (`updated_at`);
