# rinha-2024-q1-rust
Rinha de Backend - 2024 - Q1 - Implementação em rust + tokio + warp + deadpool-postgres

![image](https://github.com/ddanielvaz/rinha-2024-q1-rust/assets/4839159/f82726ac-e2b7-49d4-85f0-eb50b2f21886)

# Stack
- rust
- [warp](https://github.com/seanmonstar/warp)
- [deadpool-postgres](https://github.com/bikeshedder/deadpool)
- NGINX
- postgresql

# Uso
```bash
Usage: make <target>
  help                       Prints available commands
  cargo.build                Cargo Build
  dev.up                     Start the rinha in dev
  dev.down                   Stop the rinha in dev
  prod.up                    Start the rinha in prod
  prod.down                  Stop the rinha in prod
  docker.stats               Show docker stats
  health.check               Check the stack is healthy
  stress.it                  Run local stress tests
  docker.build               Build the docker image
  docker.push                Push the docker image
```

# Créditos
Toda parte de infra eu reutilizei do repositório [quokka](https://github.com/leandronsp/quokka/)
- config
- config/init.sql
- congig/nginx.conf
- config/postgresql.conf
- docker-compose.yml
- docker-compose-dev.yml
- Dockerfile
- Makefile

Obrigado [@leandronsp](https://github.com/leandronsp), aprendi muito lendo, reusando e modificando essa parte de infra. Uma menção especial ao statement SELECT ... FOR UPDATE.
