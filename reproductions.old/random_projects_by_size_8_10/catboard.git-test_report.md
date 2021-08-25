# Test report for javascript / file:///tmp/top-repos-quality-repos-yflfzs2n/catboard.git HEAD 4ab36c6e4daa5b0693c8db6d143e8bf04816a232

### Classification report

PPCR: 0.816

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.892| 0.969| 0.947| 0.929| 0.919| 798| 816| 0.978 |
| `␣` | 0.838| 0.702| 0.544| 0.764| 0.659| 258| 333| 0.775 |
| `'` | 0.912| 0.973| 0.747| 0.942| 0.822| 149| 194| 0.768 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 28| 0.500 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 28| 0.357 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 51| 0.157 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 26| 0.115 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 46| 0.043 |
| `macro avg` | 0.330| 0.330| 0.280| 0.329| 0.300| 1242| 1522| 0.816 |
| `micro avg` | 0.885| 0.885| 0.722| 0.885| 0.795| 1242| 1522| 0.816 |
| `weighted avg` | 0.856| 0.885| 0.722| 0.868| 0.741| 1242| 1522| 0.816 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|18 |773 |25 |0 |0 |0 |0 |0 |0 |
|75 |77 |181 |0 |0 |0 |0 |0 |0 |
|45 |1 |3 |145 |0 |0 |0 |0 |0 |
|43 |6 |2 |0 |0 |0 |0 |0 |0 |
|44 |0 |2 |0 |0 |0 |0 |0 |0 |
|23 |3 |0 |0 |0 |0 |0 |0 |0 |
|18 |7 |3 |0 |0 |0 |0 |0 |0 |
|14 |0 |0 |14 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u0027": {"f1-score": 0.9415584415584416, "precision": 0.9119496855345912, "recall": 0.9731543624161074, "support": 149}, "macro avg": {"f1-score": 0.32922500628196827, "precision": 0.33018660124673865, "recall": 0.3304220536513752, "support": 1242}, "micro avg": {"f1-score": 0.8848631239935588, "precision": 0.8848631239935588, "recall": 0.8848631239935588, "support": 1242}, "weighted avg": {"f1-score": 0.8681931950414797, "precision": 0.8563252145307811, "recall": 0.8848631239935588, "support": 1242}, "\u2205": {"f1-score": 0.9285285285285285, "precision": 0.8915801614763552, "recall": 0.968671679197995, "support": 798}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.7637130801687763, "precision": 0.8379629629629629, "recall": 0.7015503875968992, "support": 258}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u0027": {"f1-score": 0.821529745042493, "precision": 0.9119496855345912, "recall": 0.7474226804123711, "support": 194}, "macro avg": {"f1-score": 0.299938522417153, "precision": 0.33018660124673865, "recall": 0.27978376819056777, "support": 1522}, "micro avg": {"f1-score": 0.7952243125904487, "precision": 0.8848631239935588, "recall": 0.7220762155059133, "support": 1522}, "weighted avg": {"f1-score": 0.741475886074773, "precision": 0.7775882506078077, "recall": 0.7220762155059133, "support": 1522}, "\u2205": {"f1-score": 0.9185977421271538, "precision": 0.8915801614763552, "recall": 0.9473039215686274, "support": 816}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u2423": {"f1-score": 0.6593806921675773, "precision": 0.8379629629629629, "recall": 0.5435435435435435, "support": 333}},
  "ppcr": 0.8160315374507228
}
```
</details>
