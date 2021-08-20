# Train report for javascript / file:///tmp/top-repos-quality-repos-mt_qqi45/zecwallet.git HEAD 3f59978ebbf0a67a7ee70d103f5b08c34b191ae7

### Classification report

PPCR: 0.683

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.965| 0.996| 0.886| 0.980| 0.924| 14169| 15936| 0.889 |
| `␣` | 0.938| 0.869| 0.280| 0.902| 0.431| 2113| 6556| 0.322 |
| `'` | 1.000| 1.000| 0.827| 1.000| 0.905| 1640| 1984| 0.827 |
| `⏎␣⁺␣⁺` | 0.949| 0.787| 0.486| 0.861| 0.643| 541| 877| 0.617 |
| `⏎␣⁻␣⁻` | 1.000| 0.787| 0.442| 0.881| 0.613| 494| 880| 0.561 |
| `⏎` | 0.968| 0.840| 0.273| 0.900| 0.425| 475| 1464| 0.324 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 738| 0.047 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 53| 0.000 |
| `macro avg` | 0.728| 0.660| 0.399| 0.691| 0.493| 19467| 28488| 0.683 |
| `weighted avg` | 0.964| 0.966| 0.660| 0.964| 0.740| 19467| 28488| 0.683 |
| `micro avg` | 0.966| 0.966| 0.660| 0.966| 0.784| 19467| 28488| 0.683 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1767 |14113 |52 |0 |0 |4 |0 |0 |0 |
|4443 |258 |1836 |0 |0 |19 |0 |0 |0 |
|344 |0 |0 |1640 |0 |0 |0 |0 |0 |
|989 |53 |23 |0 |399 |0 |0 |0 |0 |
|336 |72 |43 |0 |0 |426 |0 |0 |0 |
|386 |96 |3 |0 |6 |0 |389 |0 |0 |
|703 |28 |0 |0 |7 |0 |0 |0 |0 |
|53 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/Routes.js | 124 |
| app/components/Sidebar.js | 88 |
| app/components/LoadingScreen.js | 47 |
| app/components/Receive.js | 43 |
| app/rpc.js | 40 |
| app/components/Transactions.js | 40 |
| app/components/Addressbook.js | 37 |
| app/companion.js | 31 |
| app/components/Dashboard.js | 26 |
| babel.config.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1640}, "macro avg": {"f1-score": 0.6905012832966511, "precision": 0.7275892255551427, "recall": 0.659979319256145, "support": 19467}, "micro avg": {"f1-score": 0.9658909950172087, "precision": 0.9658909950172087, "recall": 0.9658909950172087, "support": 19467}, "weighted avg": {"f1-score": 0.9640143155470504, "precision": 0.9640568181614967, "recall": 0.9658909950172087, "support": 19467}, "\u2205": {"f1-score": 0.980443919552607, "precision": 0.9653214774281805, "recall": 0.9960477097889759, "support": 14169}, "\u23ce": {"f1-score": 0.8996617812852311, "precision": 0.9684466019417476, "recall": 0.84, "support": 475}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8606060606060606, "precision": 0.9487750556792873, "recall": 0.7874306839186691, "support": 541}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8810872027180069, "precision": 1.0, "recall": 0.7874493927125507, "support": 494}, "\u2423": {"f1-score": 0.9022113022113022, "precision": 0.9381706693919264, "recall": 0.8689067676289636, "support": 2113}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u0027": {"f1-score": 0.9050772626931568, "precision": 1.0, "recall": 0.8266129032258065, "support": 1984}, "macro avg": {"f1-score": 0.49264404583725, "precision": 0.7275892255551427, "recall": 0.3990749919521043, "support": 28488}, "micro avg": {"f1-score": 0.7841935147534146, "precision": 0.9658909950172087, "recall": 0.6600322942993541, "support": 28488}, "weighted avg": {"f1-score": 0.7396140660070543, "precision": 0.9354075934394274, "recall": 0.6600322942993541, "support": 28488}, "\u2205": {"f1-score": 0.9237465636863464, "precision": 0.9653214774281805, "recall": 0.8856049196787149, "support": 15936}, "\u23ce": {"f1-score": 0.4253731343283582, "precision": 0.9684466019417476, "recall": 0.2725409836065574, "support": 1464}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 738}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6425339366515838, "precision": 0.9487750556792873, "recall": 0.48574686431014824, "support": 877}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6130811662726556, "precision": 1.0, "recall": 0.4420454545454545, "support": 880}, "\u2423": {"f1-score": 0.4313403030658992, "precision": 0.9381706693919264, "recall": 0.28004881025015255, "support": 6556}},
  "ppcr": 0.6833403538331929
}
```
</details>
