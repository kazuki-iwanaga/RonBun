# RonBun

## コンテナの操作
```
docker-compose up -d
docker-compose ps
docker-compose exec web bash
docker-compose down
```

## 起動するコンテナ
|container_name|host->container|
----|----
|web(rust)|8989->8989|
|db(mysql)|3306->3306|
|pma(phpmyadmin)|8080->80|

## CRUD
```
docker-compose up -d
docker-compose exec web bash
# cargo run --bin create_users
# cargo run --bin read_users
# cargo run --bin update_users
# cargo run --bin delete_users
```
