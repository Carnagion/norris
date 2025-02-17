CREATE TABLE `registrations` (
  `user_id` bigint(20) unsigned NOT NULL,
  `status` enum('UNREGISTERED','STARTED','NAME_ENTERED','KIND_FOUND','VERIFIED','PRONOUNS_PICKED','REGISTERED','FAILED') NOT NULL DEFAULT 'UNREGISTERED',
  `name` varchar(1024) DEFAULT NULL,
  `kind` enum('UNDERGRAD','POSTGRAD','MENTOR','SENIOR_MENTOR','HONORARY_MENTOR','FACULTY') DEFAULT NULL,
  PRIMARY KEY (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

CREATE TABLE `users` (
  `name` varchar(1024) NOT NULL,
  `kind` enum('UNDERGRAD','POSTGRAD','MENTOR','SENIOR_MENTOR','HONORARY_MENTOR','FACULTY') DEFAULT NULL,
  `registered_user_id` bigint(20) unsigned DEFAULT NULL,
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`id`),
  KEY `users_registrations_FK` (`registered_user_id`),
  CONSTRAINT `users_registrations_FK` FOREIGN KEY (`registered_user_id`) REFERENCES `registrations` (`user_id`) ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;