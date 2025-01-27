# Test report for javascript / file:///tmp/top-repos-quality-repos-z1fc_7qr/snickr.git HEAD 9d4a1df888a47d69d8a6b991e075b4b7e8df646b

### Classification report

PPCR: 0.958

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.925| 0.970| 0.941| 0.947| 0.933| 3607| 3718| 0.970 |
| `␣` | 0.860| 0.876| 0.866| 0.868| 0.863| 2115| 2140| 0.988 |
| `'` | 0.990| 0.840| 0.840| 0.909| 0.909| 450| 450| 1.000 |
| `⏎␣⁺␣⁺` | 0.866| 0.841| 0.811| 0.854| 0.838| 347| 360| 0.964 |
| `⏎` | 0.874| 0.869| 0.676| 0.872| 0.763| 344| 442| 0.778 |
| `⏎␣⁻␣⁻` | 0.868| 0.739| 0.702| 0.798| 0.776| 329| 346| 0.951 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 65| 118| 0.551 |
| `"` | 1.000| 0.935| 0.935| 0.966| 0.966| 46| 46| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 15| 1.000 |
| `micro avg` | 0.903| 0.903| 0.865| 0.903| 0.883| 7318| 7635| 0.958 |
| `weighted avg` | 0.893| 0.903| 0.865| 0.897| 0.874| 7318| 7635| 0.958 |
| `macro avg` | 0.709| 0.674| 0.641| 0.690| 0.672| 7318| 7635| 0.958 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|111 |3497 |106 |0 |2 |0 |2 |0 |0 |0 |
|25 |161 |1853 |4 |31 |44 |22 |0 |0 |0 |
|0 |0 |72 |378 |0 |0 |0 |0 |0 |0 |
|98 |19 |25 |0 |299 |1 |0 |0 |0 |0 |
|13 |29 |26 |0 |0 |292 |0 |0 |0 |0 |
|17 |47 |39 |0 |0 |0 |243 |0 |0 |0 |
|0 |0 |3 |0 |0 |0 |0 |43 |0 |0 |
|53 |24 |31 |0 |10 |0 |0 |0 |0 |0 |
|0 |2 |0 |0 |0 |0 |13 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9662921348314606, "precision": 1.0, "recall": 0.9347826086956522, "support": 46}, "\u0027": {"f1-score": 0.9086538461538461, "precision": 0.9895287958115183, "recall": 0.84, "support": 450}, "macro avg": {"f1-score": 0.6903710147373987, "precision": 0.7092624066660355, "recall": 0.6744106346842068, "support": 7318}, "micro avg": {"f1-score": 0.9025690079256627, "precision": 0.9025690079256627, "recall": 0.9025690079256627, "support": 7318}, "weighted avg": {"f1-score": 0.8968628199118654, "precision": 0.8929580342404343, "recall": 0.9025690079256627, "support": 7318}, "\u2205": {"f1-score": 0.9469266179258056, "precision": 0.9253770838846256, "recall": 0.9695037427224841, "support": 3607}, "\u23ce": {"f1-score": 0.8717201166180758, "precision": 0.8742690058479532, "recall": 0.8691860465116279, "support": 344}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8538011695906433, "precision": 0.8664688427299704, "recall": 0.8414985590778098, "support": 347}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7980295566502463, "precision": 0.8678571428571429, "recall": 0.7386018237082067, "support": 329}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.8679156908665105, "precision": 0.859860788863109, "recall": 0.8761229314420804, "support": 2115}},
  "cl_report_full": {"\"": {"f1-score": 0.9662921348314606, "precision": 1.0, "recall": 0.9347826086956522, "support": 46}, "\u0027": {"f1-score": 0.9086538461538461, "precision": 0.9895287958115183, "recall": 0.84, "support": 450}, "macro avg": {"f1-score": 0.6719673128449688, "precision": 0.7092624066660355, "recall": 0.6412359708663458, "support": 7635}, "micro avg": {"f1-score": 0.8834347622550658, "precision": 0.9025690079256627, "recall": 0.8650949574328749, "support": 7635}, "weighted avg": {"f1-score": 0.8743695726067683, "precision": 0.8867814406760225, "recall": 0.8650949574328749, "support": 7635}, "\u2205": {"f1-score": 0.9329064959317062, "precision": 0.9253770838846256, "recall": 0.9405594405594405, "support": 3718}, "\u23ce": {"f1-score": 0.7627551020408163, "precision": 0.8742690058479532, "recall": 0.6764705882352942, "support": 442}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8378766140602584, "precision": 0.8664688427299704, "recall": 0.8111111111111111, "support": 360}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7763578274760383, "precision": 0.8678571428571429, "recall": 0.7023121387283237, "support": 346}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.8628637951105937, "precision": 0.859860788863109, "recall": 0.8658878504672897, "support": 2140}},
  "ppcr": 0.9584806810740013
}
```
</details>
