CREATE TABLE IF NOT EXISTS `papers` (
    `paper_id`      INT(10) AUTO_INCREMENT PRIMARY KEY,
    `paper_title`   VARCHAR(50) NOT NULL,
    `paper_author`  VARCHAR(50),
    `paper_year`    INT(4),
    `user_id`       INT(10),
    `created_at`    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id)
        REFERENCES users (user_id)
        ON UPDATE CASCADE ON DELETE SET NULL
) ENGINE=INNODB DEFAULT CHARSET=utf8;

-- Sample Data --
INSERT INTO papers (paper_title, paper_author, paper_year, user_id) VALUES
('たんぽぽとは何か', 'たんぽぽ伯爵', 1998, 1),
('新型シャープペンシルの設計', '山田太郎', 2004, 2),
('たんぽぽの育て方', 'たんぽぽ伯爵', 2018, 1),
('The effective way to quit your university.', 'Taro Yamada', 2020, 2),
('シャープペンシルは本当にシャープか', '山田太郎', 2001, 2),
('What is Tanpopo?', 'Count Tanpopo', 1999, 1);
