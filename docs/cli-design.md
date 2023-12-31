# Cli Design

命令行交互接口设计方案，假设我们目前的可执行文件名为`calc`。

目前我们的输入形式包括表达式(Expression)和语句(Sentence)，前者会产生结果，而后者只用于赋值。

## 交互方式

用于在本程序中，只有表达式会产生结果，而语句只包含赋值操作。从严格意义上讲，我们只应该输出表达式的计算结果，而对于语句来说，应该返回错误（未输入表达式）或不予理睬，当然也可以返回赋值结果。

> 在实现上，请选择上述的任何一种方式来实现，但是请保证整个程序的处理方式统一。

### 1. 单条表达式输入

直接在执行程序时携带表达式，程序可以直接计算并显示结果。值得注意的是，在输入时需要使用双引号包括，保证表达式不可分。

> 在具体实现上，可以自行设计提示词`[ouput]`

````bash
$ calc -e "1+1"
$ [output] 2
````

### 2. 文件输入

通过输入文件路径，程序会逐行读取文件中的内容，并解析运行。在文件中，可能会出现多条表达式，你都需要将结果输出出来。

```bash
$ calc -p text.calc
```

### 3. 多条表达式交互输入

除了上述通过文件进行多行输入的方法，我们还应该提供一个交互接口，能够让用户通过交互的方式进行多行输入。

```
$ calc -i
> 1 + 1
2
> a = 2
> a + 1
3
```

## 错误提示

请尽量处理掉任何可能会出现`panic!`的情况，需要捕获可能会出现错误（文件读取错误，解析错误，计算错误等），并进行提示。



