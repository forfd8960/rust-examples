
### logs

server

```
2022-05-15T02:55:05.147405Z  INFO server: start listen on: 127.0.0.1:9527
2022-05-15T02:55:40.194140Z  INFO server: client: 127.0.0.1:49562 connected
2022-05-15T02:55:40.194401Z  INFO server: got a new command: CommondRequest { request_data: Some(Hset(Hset { table: "table1", pairs: Some(KVpair { key: "hello", value: Some(Value { value: Some(String("world")) }) }) })) }
2022-05-15T02:55:40.194955Z  INFO server: client disconnected: 127.0.0.1:49562
```

client

```sh
2022-05-15T02:55:40.194736Z  INFO client: Got response: CommandResponse { status: 404, message: "Not Found", values: [], pairs: [] }
```

### logs1

* server:

```sh
2022-05-19T01:11:09.442551Z  INFO server: start listen on: 127.0.0.1:9527
2022-05-19T01:11:26.708682Z  INFO server: client: 127.0.0.1:54189 connected
2022-05-19T01:11:26.708874Z  INFO server: got a new command: CommondRequest { request_data: Some(Hset(Hset { table: "table1", pairs: Some(KVpair { key: "hello", value: Some(Value { value: Some(String("world")) }) }) })) }
2022-05-19T01:11:26.709361Z  INFO server: client disconnected: 127.0.0.1:54189
2022-05-19T01:11:52.517507Z  INFO server: client: 127.0.0.1:54296 connected
2022-05-19T01:11:52.517652Z  INFO server: got a new command: CommondRequest { request_data: Some(Hget(Hget { table: "table1", key: "hello" })) }
2022-05-19T01:11:52.518053Z  INFO server: client disconnected: 127.0.0.1:54296
2022-05-19T01:12:52.262258Z  INFO server: client: 127.0.0.1:54541 connected
2022-05-19T01:12:52.262440Z  INFO server: got a new command: CommondRequest { request_data: Some(Hgetall(Hgetall { table: "table1" })) }
2022-05-19T01:12:52.263039Z  INFO server: client disconnected: 127.0.0.1:54541
```

* client

```sh
2022-05-19T01:12:52.262749Z  INFO client: Got response: CommandResponse { status: 200, message: "", values: [], pairs: [KVpair { key: "hello", value: Some(Value { value: Some(String("world")) }) }] }
```