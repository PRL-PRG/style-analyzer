# Test report for javascript / file:///tmp/top-repos-quality-repos-wyfvz12f/contrihub-18.git HEAD 48ca8193be096ee4f139073492ae08adab17df3d

### Classification report

PPCR: 0.937

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.926| 0.926| 0.953| 0.953| 1883| 1884| 0.999 |
| `␣` | 0.790| 0.978| 0.957| 0.874| 0.865| 941| 961| 0.979 |
| `⏎` | 0.818| 0.785| 0.681| 0.801| 0.743| 223| 257| 0.868 |
| `'` | 0.746| 0.890| 0.630| 0.811| 0.683| 191| 270| 0.707 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.983| 0.760| 0.760| 0.857| 0.857| 150| 150| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.989| 0.845| 0.608| 0.911| 0.753| 103| 143| 0.720 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 62| 124| 0.500 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 39| 0.897 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.882| 0.895| 0.839| 0.884| 0.846| 3588| 3828| 0.937 |
| `micro avg` | 0.895| 0.895| 0.839| 0.895| 0.866| 3588| 3828| 0.937 |
| `macro avg` | 0.531| 0.518| 0.456| 0.521| 0.485| 3588| 3828| 0.937 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |1744 |139 |0 |0 |0 |0 |0 |0 |
|20 |18 |920 |0 |1 |0 |0 |1 |1 |
|79 |10 |11 |170 |0 |0 |0 |0 |0 |
|34 |2 |45 |0 |175 |0 |0 |1 |0 |
|62 |0 |4 |58 |0 |0 |0 |0 |0 |
|4 |0 |4 |0 |31 |0 |0 |0 |0 |
|0 |0 |35 |0 |1 |0 |0 |114 |0 |
|40 |3 |7 |0 |6 |0 |0 |0 |87 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u0027": {"f1-score": 0.8114558472553699, "precision": 0.7456140350877193, "recall": 0.8900523560209425, "support": 191}, "macro avg": {"f1-score": 0.5207208472112328, "precision": 0.5305894974926699, "recall": 0.5183330854112462, "support": 3588}, "micro avg": {"f1-score": 0.8946488294314381, "precision": 0.8946488294314381, "recall": 0.8946488294314381, "support": 3588}, "weighted avg": {"f1-score": 0.8842395801431787, "precision": 0.8821500117805132, "recall": 0.8946488294314381, "support": 3588}, "\u2205": {"f1-score": 0.9530054644808742, "precision": 0.9814293753517164, "recall": 0.9261816250663835, "support": 1883}, "\u23ce": {"f1-score": 0.8009153318077803, "precision": 0.8177570093457944, "recall": 0.7847533632286996, "support": 223}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8571428571428572, "precision": 0.9827586206896551, "recall": 0.76, "support": 150}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9109947643979057, "precision": 0.9886363636363636, "recall": 0.8446601941747572, "support": 103}, "\u2423": {"f1-score": 0.8736942070275403, "precision": 0.7896995708154506, "recall": 0.9776833156216791, "support": 941}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u0027": {"f1-score": 0.6827309236947792, "precision": 0.7456140350877193, "recall": 0.6296296296296297, "support": 270}, "macro avg": {"f1-score": 0.4854440543923122, "precision": 0.5305894974926699, "recall": 0.4561981219613342, "support": 3828}, "micro avg": {"f1-score": 0.8656957928802589, "precision": 0.8946488294314381, "recall": 0.8385579937304075, "support": 3828}, "weighted avg": {"f1-score": 0.8459490902644786, "precision": 0.8642064693561347, "recall": 0.8385579937304075, "support": 3828}, "\u2205": {"f1-score": 0.9527451515979241, "precision": 0.9814293753517164, "recall": 0.9256900212314225, "support": 1884}, "\u23ce": {"f1-score": 0.743099787685775, "precision": 0.8177570093457944, "recall": 0.6809338521400778, "support": 257}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8571428571428572, "precision": 0.9827586206896551, "recall": 0.76, "support": 150}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7532467532467533, "precision": 0.9886363636363636, "recall": 0.6083916083916084, "support": 143}, "\u2423": {"f1-score": 0.8654750705550329, "precision": 0.7896995708154506, "recall": 0.9573361082206036, "support": 961}},
  "ppcr": 0.9373040752351097
}
```
</details>