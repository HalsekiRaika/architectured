### Layered Architecture Test  

#### How to run

Required Programs  
+ docker
+ rust-toolchain

```shell
# Copy Redis Connection URL Environment Value 
cat .env.template > .env

# Redis
docker-compose up -d

# run program
cargo run
```