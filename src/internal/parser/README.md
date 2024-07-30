# Internal Parsing

$S\rightarrow R;S\ |$ \
$R\rightarrow n:R'\ |\ t:p$\
$R' \rightarrow R'' R'''$\
$R'' \rightarrow t R''\ |\ nR''\ |$ \
$R''' \rightarrow bR'\ |$

## Table-driven LL(1)
*If this is enough, ignore SLR*

|| Nullable | First | Follow |
|-|-|-|-|
|S|y|${n,t}$|$Ø$|
|R|n|${n,t}$|$\{;\}$|
|R'|y|${n,t,b}$|$\{;\}$|
|R''|y|${n,t}$|$\{b,;\}$|
|R'''|y|${b}$|$\{;\}$|

<details>
<summary>Nullable</summary>

|Rule \ Iteration| 1|2|3|4|
|-|-|-|-|-|
|$S$|n|y|y|y|
|$R$|n|n|n|n|
|$R'$|n|n|y|y|
|$R''$|n|y|y|y|
|$R'''$|n|y|y|y|

</details>
<details>
<summary>Follow</summary>

$S'\rightarrow S\$$

|Rule \ Iteration | 1 | 2 |
|-|-|-|
|$S$|$Ø$|$Ø$|
|$R$|$\{;\}$|$\{;\}$|
|$R'$|$Ø$|$\{;\}$|
|$R''$|$\{b\}$|$\{b,;\}$|
|$R'''$|$Ø$|$\{;\}$|

</details>

## SLR

add additional

$S'\rightarrow S$

*Yes the below NFAs skip N. I forgot*

```mermaid
---
title: NFA
---
stateDiagram-v2
[*] --> A : S'->S
A --> (B) : S

[*] --> C : S->R SEMI S
C --> D : R
D --> E : SEMI
E --> (F) : S

[*] --> (G) : S->
```

```mermaid
---
title: Combined NFA
---
stateDiagram-v2
[*] --> A : S->P
A --> (B) : P
A --> C : e
A --> (G) : e

[*] --> C : P->R SEMI P
C --> D : R
C --> H : e
D --> E : SEMI
E --> (F) : P

[*] --> (G) : P->

[*] --> H : R->R'R''
H --> I : R'
H --> K : e
H --> O : e
I --> (J) : R''
I --> R : e
I --> (U) : e

[*] --> K : R'->tR'
K --> L : t
L --> (M) : R'
L --> K : e
L --> O : e

[*] --> O : R'->nR'
O --> P : n
P --> (Q) : R'
P --> K : e
P --> O : e

[*] --> R : R''->bR
R --> S : b
S --> (T) : R
S --> H : e

[*] --> (U) : R''->
```
