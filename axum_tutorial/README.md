To auto start the server, we run
```bash
cargo install cargo-watch
cargo watch -x run
```
To create a database with Docker
```bash
sudo docker-compose up -d #inside backend/data
sudo docker-compose exec database /bin/bash
psql -U postgres -d postgres
\dt # to show a list of created tables
```

```bash
sudo docker-compose down #remove the database container
sudo docker volume ls
sudo docker volume rm xxx
sudo docker-compose logs [service_name]
```

Use sea-orm-cli
```bash
cargo install sea-orm-cli
sea-orm-cli generate entity -o src/database
```