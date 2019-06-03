in order to run test on high load I use two seperated tools:
1. the old ApacheBench - test only HTTP1.0 request, interesting for a lot of different connections
2. [autocannon](https://github.com/mcollina/autocannon) - test HTTP1/1.1 with many advanced capabilities

# Install
npm i autocannon -g


# testing
## autocannon
```
ulimit -n 2048
pm2 start node/app.js -i max --name="${noamServer}"
autocannon -c 1000 http://localhost:3000/json
```

```
ulimit -n 2048
cargo run& 
autocannon -c 1000 http://localhost:8080/home/noam2/page2
```

## ApacheBench
```
ulimit -n 2048
ab -n 1000000 -c 1000 -r http://localhost:8080/home/noam2/page2
```

```
ulimit -n 2048
ab -n 1000000 -c 1000 -r http://localhost:3000/json
```
