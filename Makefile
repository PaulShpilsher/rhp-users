.PHONY: dev
dev:	export PORT=8181
dev:	export RUST_LOG=debug
dev:	export RUST_BACKTRACE=1
dev:	export PG.USER=svc
dev:	export PG.PASSWORD=secret
dev:	export PG.HOST=127.0.0.1
dev:	export PG.PORT=5432
dev:	export PG.DBNAME=users
dev:	export PG.POOL.MAX_SIZE=16
dev:
		cargo run


.PHONY: postgres
postgres:
		docker run --rm -ti --network host \
			-e POSTGRES_USER=svc \
			-e POSTGRES_PASSWORD=secret \
			-e POSTGRES_DB=users \
			--name postgresql-users \
			postgres

.PHONY: adminer
adminer:
		docker run --rm -ti --network host adminer