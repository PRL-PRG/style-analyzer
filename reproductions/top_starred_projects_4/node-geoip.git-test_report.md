# Test report for javascript / file:///tmp/top-repos-quality-repos-ef8spp88/node-geoip.git HEAD 78f4c7f0f3b6fa673035b3cb48c3a2a95be9fb0a

### Classification report

PPCR: 0.850

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.749| 0.997| 0.991| 0.855| 0.853| 1705| 1716| 0.994 |
| `␣` | 0.904| 0.653| 0.536| 0.758| 0.673| 881| 1073| 0.821 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 112| 164| 0.683 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 83| 129| 0.643 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 64| 143| 0.448 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 128| 0.258 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 66| 0.424 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.207| 0.206| 0.191| 0.202| 0.191| 2906| 3419| 0.850 |
| `weighted avg` | 0.713| 0.783| 0.665| 0.732| 0.639| 2906| 3419| 0.850 |
| `micro avg` | 0.783| 0.783| 0.665| 0.783| 0.719| 2906| 3419| 0.850 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|11 |1700 |5 |0 |0 |0 |0 |0 |
|192 |306 |575 |0 |0 |0 |0 |0 |
|79 |38 |26 |0 |0 |0 |0 |0 |
|52 |100 |12 |0 |0 |0 |0 |0 |
|38 |17 |11 |0 |0 |0 |0 |0 |
|46 |76 |7 |0 |0 |0 |0 |0 |
|95 |33 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "macro avg": {"f1-score": 0.20167763253359205, "precision": 0.20662334109107028, "recall": 0.20621685900785897, "support": 2906}, "micro avg": {"f1-score": 0.782863041982106, "precision": 0.782863041982106, "recall": 0.782863041982106, "support": 2906}, "weighted avg": {"f1-score": 0.7316686116804189, "precision": 0.7134803231325118, "recall": 0.782863041982106, "support": 2906}, "\u2205": {"f1-score": 0.8553459119496856, "precision": 0.748898678414097, "recall": 0.9970674486803519, "support": 1705}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.7580751483190508, "precision": 0.9040880503144654, "recall": 0.6526674233825198, "support": 881}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "macro avg": {"f1-score": 0.19073669781038788, "precision": 0.20662334109107028, "recall": 0.1908195873713115, "support": 3419}, "micro avg": {"f1-score": 0.7193675889328064, "precision": 0.782863041982106, "recall": 0.6653992395437263, "support": 3419}, "weighted avg": {"f1-score": 0.6392961268667756, "precision": 0.6596070810605476, "recall": 0.6653992395437263, "support": 3419}, "\u2205": {"f1-score": 0.8529854490717512, "precision": 0.748898678414097, "recall": 0.9906759906759907, "support": 1716}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 143}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u2423": {"f1-score": 0.6729081334113518, "precision": 0.9040880503144654, "recall": 0.5358807082945014, "support": 1073}},
  "ppcr": 0.8499561275226675
}
```
</details>
