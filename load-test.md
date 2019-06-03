in order to run test on high load I use two seperated tools:
1. the old ab tool (Apache Bench) - test only HTTP1.0 request, interesting for a lot of different connections
2. autocannon - test HTTP1/1.1 with many advanced capabilities

# Install

for mor info look at [autocannon](https://github.com/mcollina/autocannon)

## Using NPM
npm i autocannon -g


# testing
```
node node/app.js&
autocannon -c 1000 http://localhost:3000/json
```

```
cargo run& 
autocannon -c 1000 http://localhost:8080/home/noam2/page2
```