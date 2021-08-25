# Test report for javascript / file:///tmp/top-repos-quality-repos-l_vk29hn/sidoine.org.git HEAD ed247b93ac65ff3f878ddf81ccacff09282ed1b1

### Classification report

PPCR: 0.625

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.948| 1.000| 0.898| 0.973| 0.922| 326| 363| 0.898 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 39| 78| 0.500 |
| `␣` | 0.833| 0.417| 0.037| 0.556| 0.071| 12| 135| 0.089 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 11| 0.636 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 25| 0.160 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 10| 0.100 |
| `micro avg` | 0.951| 0.951| 0.595| 0.951| 0.732| 389| 622| 0.625 |
| `weighted avg` | 0.920| 0.951| 0.595| 0.933| 0.637| 389| 622| 0.625 |
| `macro avg` | 0.464| 0.403| 0.239| 0.421| 0.277| 389| 622| 0.625 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|37 |326 |0 |0 |0 |0 |0 |
|123 |7 |5 |0 |0 |0 |0 |
|39 |0 |0 |39 |0 |0 |0 |
|21 |3 |1 |0 |0 |0 |0 |
|4 |7 |0 |0 |0 |0 |0 |
|9 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 39}, "macro avg": {"f1-score": 0.4214483139856274, "precision": 0.4635012919896641, "recall": 0.40277777777777785, "support": 389}, "micro avg": {"f1-score": 0.9511568123393316, "precision": 0.9511568123393316, "recall": 0.9511568123393316, "support": 389}, "weighted avg": {"f1-score": 0.9329266265075652, "precision": 0.9201590243319185, "recall": 0.9511568123393316, "support": 389}, "\u2205": {"f1-score": 0.973134328358209, "precision": 0.9476744186046512, "recall": 1.0, "support": 326}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.5555555555555556, "precision": 0.8333333333333334, "recall": 0.4166666666666667, "support": 12}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 78}, "macro avg": {"f1-score": 0.27663252647453196, "precision": 0.4635012919896641, "recall": 0.23918477706356492, "support": 622}, "micro avg": {"f1-score": 0.7319485657764591, "precision": 0.9511568123393316, "recall": 0.594855305466238, "support": 622}, "weighted avg": {"f1-score": 0.637195224912497, "precision": 0.8593341060345471, "recall": 0.594855305466238, "support": 622}, "\u2205": {"f1-score": 0.9222065063649223, "precision": 0.9476744186046512, "recall": 0.8980716253443526, "support": 363}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.07092198581560283, "precision": 0.8333333333333334, "recall": 0.037037037037037035, "support": 135}},
  "ppcr": 0.6254019292604501
}
```
</details>
