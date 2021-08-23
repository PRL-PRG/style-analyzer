# Train report for javascript / file:///tmp/top-repos-quality-repos-3r43hcge/vs_win_repos.git HEAD 84dbbacbbb5a990a0b04097ebc70ddbc883e56b9

### Classification report

PPCR: 0.872

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.956| 0.998| 0.937| 0.976| 0.946| 99642| 106128| 0.939 |
| `∅` | 0.999| 0.978| 0.957| 0.988| 0.977| 81822| 83574| 0.979 |
| `"` | 1.000| 1.000| 0.987| 1.000| 0.993| 9822| 9954| 0.987 |
| `⏎` | 0.992| 0.834| 0.567| 0.906| 0.722| 9480| 13932| 0.680 |
| `⏎⇥⁺` | 0.935| 0.924| 0.762| 0.929| 0.840| 7440| 9024| 0.824 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 612| 6090| 0.100 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 288| 8220| 0.035 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 108| 1194| 0.090 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 366| 0.098 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 210| 0.143 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 1242| 0.019 |
| `macro avg` | 0.444| 0.430| 0.383| 0.436| 0.407| 209304| 239934| 0.872 |
| `weighted avg` | 0.970| 0.975| 0.850| 0.972| 0.874| 209304| 239934| 0.872 |
| `micro avg` | 0.975| 0.975| 0.850| 0.975| 0.908| 209304| 239934| 0.872 |

### Confusion matrix

|refusal|  ␣| ∅| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎⇥⁻⇥⁻| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6486 |99444 |48 |0 |0 |150 |0 |0 |0 |0 |0 |0 |
|1752 |1824 |79998 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4452 |1524 |24 |7902 |0 |30 |0 |0 |0 |0 |0 |0 |
|132 |0 |0 |0 |9822 |0 |0 |0 |0 |0 |0 |0 |
|1584 |546 |12 |6 |0 |6876 |0 |0 |0 |0 |0 |0 |
|7932 |246 |30 |0 |0 |12 |0 |0 |0 |0 |0 |0 |
|5478 |408 |0 |48 |0 |156 |0 |0 |0 |0 |0 |0 |
|1218 |6 |0 |0 |0 |18 |0 |0 |0 |0 |0 |0 |
|1086 |0 |0 |0 |0 |108 |0 |0 |0 |0 |0 |0 |
|330 |24 |6 |0 |0 |6 |0 |0 |0 |0 |0 |0 |
|180 |18 |0 |12 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| WebAppFrameworkMVC432/Scripts/jquery-3.4.1.slim.js | 877 |
| WebAppFramework12/packages/jQuery.3.4.1/Content/Scripts/jquery-3.4.1.slim.js | 877 |
| WebAppFrameworkMVC432/packages/jQuery.3.4.1/Content/Scripts/jquery-3.4.1.slim.js | 877 |
| WebAppSQL/packages/jQuery.3.4.1/Content/Scripts/jquery-3.4.1.slim.js | 877 |
| WebAppFramework12/Scripts/jquery-3.4.1.slim.js | 877 |
| WebAppSQL/Scripts/jquery-3.4.1.slim.js | 877 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9822}, "macro avg": {"f1-score": 0.43633422000865507, "precision": 0.44370826406483205, "recall": 0.4303144041173267, "support": 209304}, "micro avg": {"f1-score": 0.9748595344570576, "precision": 0.9748595344570576, "recall": 0.9748595344570576, "support": 209304}, "weighted avg": {"f1-score": 0.9720804830601322, "precision": 0.9704435914905603, "recall": 0.9748595344570576, "support": 209304}, "\u2205": {"f1-score": 0.9879955539088552, "precision": 0.998502209241369, "recall": 0.9777077069736746, "support": 81822}, "\u23ce": {"f1-score": 0.905777166437414, "precision": 0.9917168674698795, "recall": 0.8335443037974684, "support": 9480}, "\u23ce\u21e5\u207a": {"f1-score": 0.929440389294404, "precision": 0.9347471451876019, "recall": 0.9241935483870968, "support": 7440}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 288}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 612}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u2423": {"f1-score": 0.976463310454532, "precision": 0.9558246828143022, "recall": 0.9980128861323538, "support": 99642}},
  "cl_report_full": {"\"": {"f1-score": 0.9933252427184467, "precision": 1.0, "recall": 0.9867389993972272, "support": 9954}, "macro avg": {"f1-score": 0.4071162890408923, "precision": 0.44370826406483205, "recall": 0.3827383243842391, "support": 239934}, "micro avg": {"f1-score": 0.908391543012835, "precision": 0.9748595344570576, "recall": 0.8504088624371702, "support": 239934}, "weighted avg": {"f1-score": 0.8737262972653056, "precision": 0.9048085857303163, "recall": 0.8504088624371702, "support": 239934}, "\u2205": {"f1-score": 0.9774210101898688, "precision": 0.998502209241369, "recall": 0.9572115729772417, "support": 83574}, "\u23ce": {"f1-score": 0.7216438356164383, "precision": 0.9917168674698795, "recall": 0.5671834625322998, "support": 13932}, "\u23ce\u21e5\u207a": {"f1-score": 0.8395604395604396, "precision": 0.9347471451876019, "recall": 0.761968085106383, "support": 9024}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8220}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 366}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6090}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1194}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1242}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 210}, "\u2423": {"f1-score": 0.9463286513646225, "precision": 0.9558246828143022, "recall": 0.937019448213478, "support": 106128}},
  "ppcr": 0.8723398934707044
}
```
</details>
