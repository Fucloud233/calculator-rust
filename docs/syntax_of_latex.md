# Syntax of Latex

Latex是一个十分强大的论文排版的语言，而表述公式也是其中的一项功能。我们希望借用Latex语法，来设计一个计算器。此计算器能够解析计算具有规范描述的Latex公式，而使用本计算器编写出来的表达式，也能被Latex正常渲染出来。

## 混淆点

由于Latex是一种排版语言，而非严格的编程语言，所以在理解上面可能有所偏差，在一些容易被混淆的地方，我在此对其详细描述。

### 携带表达式

在数学中有一些函数不能直接使用线性方式书写，比如说是Log函数。Log函数包括底数和真数两个部分，如果直接使用线性排列`\log a N`，结果是不直观的，因此Latex规定了一种格式分别描述底数和真数

```latex
\log_{a}{N}
```

通过大括号，我们的能够严格地区分Log函数的底数和真数部分。但是在实际场景下，如果只是想携带当个字符，但仍然使用大括号就有点臃肿，因此Latex也有更好的书写方式，当只有单字符时，是不需要使用的大括号围住的。

> 当然这种携带型也适用于最小Latex语法单元（`\alpha`, `\beta`等）

```latex
\log_aN
```

#### 存在的问题

但是值得注意的是，现在有下列两种式子（`\log_210`和`\log_2{10}`），可以发现这两种式子被渲染出来的结果是相同的，但他们是一定等价的吗？显然是不同的。因为Latex是一种排版语言，无论代码怎么写，只要“看起来”是对的就可以了，但在计算器的语言时是更加严格的，从逻辑上来说，得是分明的。
$$
\log_210 \quad \log_2{10}
$$

#### 解决方案

因此，我们在这里规范一下这种表达式的合法情况。这里以Log函数为例，但实际上包括但不限于该函数的情况。

* 标准形式: `\log_{a}{N}`
* 单字符形式：`\log_aN`（此处单字符表示单数字或Latex语法单元）

> 在实现情况下，对连续的单数字需要额外处理