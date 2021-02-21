CREATE TABLE IF NOT EXISTS `users` (
    `user_id`     INT(10) AUTO_INCREMENT PRIMARY KEY,
    `user_name`   VARCHAR(20) UNIQUE NOT NULL,
    `user_email`  VARCHAR(50) NOT NULL,
    `created_at`  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=INNODB DEFAULT CHARSET=utf8;

-- Sample Data --
INSERT INTO users (user_name, user_email) VALUES
('kazuki-iwanaga', 'kazuki-iwanaga@hogehoge.jp'),
('kygkygkyg', 'kygkygkyg@fugafuga.jp');
