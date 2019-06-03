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
Running 10s test @ http://localhost:3000/json
1000 connections

┌─────────┬───────┬───────┬───────┬───────┬──────────┬──────────┬───────────┐
│ Stat    │ 2.5%  │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev    │ Max       │
├─────────┼───────┼───────┼───────┼───────┼──────────┼──────────┼───────────┤
│ Latency │ 30 ms │ 32 ms │ 41 ms │ 53 ms │ 33.99 ms │ 12.15 ms │ 288.91 ms │
└─────────┴───────┴───────┴───────┴───────┴──────────┴──────────┴───────────┘
┌───────────┬─────────┬─────────┬─────────┬────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%  │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 20975   │ 20975   │ 30991   │ 31007  │ 29691.2 │ 2942.66 │ 20973   │
├───────────┼─────────┼─────────┼─────────┼────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 6.36 MB │ 6.36 MB │ 9.39 MB │ 9.4 MB │ 9 MB    │ 891 kB  │ 6.35 MB │
└───────────┴─────────┴─────────┴─────────┴────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.

297k requests in 10.34s, 90 MB read
```

```
ulimit -n 2048
cargo run& 
autocannon -c 1000 http://localhost:8080/home/noam2/page2
Running 10s test @ http://localhost:8080/home/noam2/page2
1000 connections

┌─────────┬───────┬───────┬───────┬───────┬──────────┬──────────┬───────────┐
│ Stat    │ 2.5%  │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev    │ Max       │
├─────────┼───────┼───────┼───────┼───────┼──────────┼──────────┼───────────┤
│ Latency │ 27 ms │ 30 ms │ 40 ms │ 43 ms │ 31.13 ms │ 10.59 ms │ 212.35 ms │
└─────────┴───────┴───────┴───────┴───────┴──────────┴──────────┴───────────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 25007   │ 25007   │ 33023   │ 34015   │ 31966.4 │ 2509.36 │ 25000   │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 6.45 MB │ 6.45 MB │ 8.52 MB │ 8.77 MB │ 8.25 MB │ 647 kB  │ 6.45 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.

320k requests in 10.28s, 82.5 MB read
```

## ApacheBench
```
ulimit -n 2048
ab -n 1000000 -c 1000 -r http://localhost:8080/home/noam2/page2
This is ApacheBench, Version 2.3 <$Revision: 1807734 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100000 requests
Completed 200000 requests
Completed 300000 requests
Completed 400000 requests
Completed 500000 requests
Completed 600000 requests
Completed 700000 requests
Completed 800000 requests
Completed 900000 requests
Completed 1000000 requests
Finished 1000000 requests


Server Software:        Noam's
Server Hostname:        localhost
Server Port:            8080

Document Path:          /home/noam2/page2
Document Length:        83 bytes

Concurrency Level:      1000
Time taken for tests:   38.535 seconds
Complete requests:      1000000
Failed requests:        0
Total transferred:      258000000 bytes
HTML transferred:       83000000 bytes
Requests per second:    25950.54 [#/sec] (mean)
Time per request:       38.535 [ms] (mean)
Time per request:       0.039 [ms] (mean, across all concurrent requests)
Transfer rate:          6538.32 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0   22  93.2     14    3062
Processing:     2   16  13.6     16     226
Waiting:        1   12  13.3     12     223
Total:          4   38  94.3     30    3080

Percentage of the requests served within a certain time (ms)
  50%     30
  66%     36
  75%     37
  80%     37
  90%     39
  95%     41
  98%     42
  99%    225
 100%   3080 (longest request)
```

```
ulimit -n 2048
ab -n 1000000 -c 1000 -r http://localhost:3000/json
This is ApacheBench, Version 2.3 <$Revision: 1807734 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100000 requests
Completed 200000 requests
Completed 300000 requests
Completed 400000 requests
Completed 500000 requests
Completed 600000 requests
Completed 700000 requests
Completed 800000 requests
Completed 900000 requests
Completed 1000000 requests
Finished 1000000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            3000

Document Path:          /json
Document Length:        91 bytes

Concurrency Level:      1000
Time taken for tests:   70.446 seconds
Complete requests:      1000000
Failed requests:        0
Total transferred:      298000000 bytes
HTML transferred:       91000000 bytes
Requests per second:    14195.27 [#/sec] (mean)
Time per request:       70.446 [ms] (mean)
Time per request:       0.070 [ms] (mean, across all concurrent requests)
Transfer rate:          4131.05 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   1.2      0    1017
Processing:    14   70   3.3     70     209
Waiting:       10   70   3.3     70     209
Total:         34   70   3.2     71    1092

Percentage of the requests served within a certain time (ms)
  50%     71
  66%     72
  75%     72
  80%     73
  90%     74
  95%     75
  98%     76
  99%     77
 100%   1092 (longest request)
```
