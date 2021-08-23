# Test report for javascript / file:///tmp/top-repos-quality-repos-mlrr7gjb/naumachia-challenges.git HEAD 8dc3053b9fb4c4adaf1627fbae31eb393aa61ebd

### Classification report

PPCR: 0.899

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.977| 0.971| 0.983| 0.980| 514| 517| 0.994 |
| `␣` | 0.902| 0.987| 0.953| 0.943| 0.927| 225| 233| 0.966 |
| `⏎` | 0.727| 0.533| 0.178| 0.615| 0.286| 15| 45| 0.333 |
| `"` | 1.000| 0.923| 0.857| 0.960| 0.923| 13| 14| 0.929 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 10| 0.500 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 24| 0.125 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 18| 0.111 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `macro avg` | 0.226| 0.214| 0.185| 0.219| 0.195| 777| 864| 0.899 |
| `micro avg` | 0.958| 0.958| 0.861| 0.958| 0.907| 777| 864| 0.899 |
| `weighted avg` | 0.947| 0.958| 0.861| 0.951| 0.867| 777| 864| 0.899 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎⇥⁺| ⏎␣⁻␣⁻| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⏎| ⏎⏎⇥⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3 |502 |12 |0 |0 |0 |0 |0 |0 |0 |
|8 |3 |222 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |12 |0 |1 |0 |0 |0 |0 |
|30 |0 |7 |0 |8 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5 |0 |2 |0 |3 |0 |0 |0 |0 |0 |
|21 |0 |3 |0 |0 |0 |0 |0 |0 |0 |
|16 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9600000000000001, "precision": 1.0, "recall": 0.9230769230769231, "support": 13}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.21883808948869432, "precision": 0.22624061367025722, "recall": 0.2137331637234361, "support": 777}, "micro avg": {"f1-score": 0.9575289575289575, "precision": 0.9575289575289575, "recall": 0.9575289575289575, "support": 777}, "weighted avg": {"f1-score": 0.9514213691878901, "precision": 0.9470898814238401, "recall": 0.9575289575289575, "support": 777}, "\u2205": {"f1-score": 0.9833496571988247, "precision": 0.9901380670611439, "recall": 0.9766536964980544, "support": 514}, "\u23ce": {"f1-score": 0.6153846153846153, "precision": 0.7272727272727273, "recall": 0.5333333333333333, "support": 15}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9426751592356688, "precision": 0.9024390243902439, "recall": 0.9866666666666667, "support": 225}},
  "cl_report_full": {"\"": {"f1-score": 0.923076923076923, "precision": 1.0, "recall": 0.8571428571428571, "support": 14}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19476194157893906, "precision": 0.22624061367025722, "recall": 0.18491854967747579, "support": 864}, "micro avg": {"f1-score": 0.9067641681901281, "precision": 0.9575289575289575, "recall": 0.8611111111111112, "support": 864}, "weighted avg": {"f1-score": 0.8665017492339735, "precision": 0.8899270209268645, "recall": 0.8611111111111112, "support": 864}, "\u2205": {"f1-score": 0.9804687500000001, "precision": 0.9901380670611439, "recall": 0.9709864603481625, "support": 517}, "\u23ce": {"f1-score": 0.2857142857142857, "precision": 0.7272727272727273, "recall": 0.17777777777777778, "support": 45}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9269311064718163, "precision": 0.9024390243902439, "recall": 0.9527896995708155, "support": 233}},
  "ppcr": 0.8993055555555556
}
```
</details>
