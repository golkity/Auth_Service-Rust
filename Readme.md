# Authorization system

## Http request

| Api    | Body                         | Type |
|--------|------------------------------|------|
| /login | username: ""<br>password: "" | Post |
| /info  | Header -> auth bear token    | Get  |

> [!IMPORTANT]<br>
> **Run**
> ```shell
> corgo run --quite
> ```
