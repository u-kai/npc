# npc

## What is is?

- npc stands for "Naming Principal Convertor"

## How to Use

```bash
$ npc --snake "HelloWorld"
hello_world
$ npc --camel "hello_world"
helloWorld
$ npc --chain "hello_world"
hello-world
$ npc --pascal "hello_world"
HelloWorld
$ npc --snake -f hello.py
$ npc --snake -f hello.py -o hello_snake.py
```

## Use Case

- 命名規則を変更したい時
- 命名規則を変更したいけど、特定の文字列だけは変更したくない時
- GitHub などを git_hub にするのではなく github など、ルール外の変換をしたい時
