# Test report for javascript / file:///tmp/top-repos-quality-repos-ks4hzi1_/studies.git HEAD cdb8206b5bd94bbbb5a310e0bb00302f988e180c

### Classification report

PPCR: 0.708

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 0.993| 0.937| 0.969| 0.941| 2194| 2325| 0.944 |
| `␣` | 0.905| 0.901| 0.661| 0.903| 0.764| 614| 837| 0.734 |
| `⏎` | 0.727| 0.186| 0.066| 0.296| 0.121| 86| 242| 0.355 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 55| 0.200 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 79| 0.114 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 61| 0.148 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 354| 0.014 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 57| 0.070 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 46| 0.065 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 90| 0.011 |
| `weighted avg` | 0.917| 0.936| 0.663| 0.921| 0.689| 2936| 4146| 0.708 |
| `micro avg` | 0.936| 0.936| 0.663| 0.936| 0.776| 2936| 4146| 0.708 |
| `macro avg` | 0.258| 0.208| 0.166| 0.217| 0.183| 2936| 4146| 0.708 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|131 |2178 |16 |0 |0 |0 |0 |0 |0 |0 |0 |
|223 |56 |553 |5 |0 |0 |0 |0 |0 |0 |0 |
|156 |44 |26 |16 |0 |0 |0 |0 |0 |0 |0 |
|349 |0 |4 |1 |0 |0 |0 |0 |0 |0 |0 |
|89 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|53 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|43 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|52 |7 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|70 |0 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|44 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "macro avg": {"f1-score": 0.21677992029960008, "precision": 0.25780693470663074, "recall": 0.20794053611998814, "support": 2936}, "micro avg": {"f1-score": 0.9356267029972752, "precision": 0.9356267029972752, "recall": 0.9356267029972752, "support": 2936}, "weighted avg": {"f1-score": 0.9213367074476659, "precision": 0.9172945748502923, "recall": 0.9356267029972752, "support": 2936}, "\u2205": {"f1-score": 0.9686457638425617, "precision": 0.9457229700390795, "recall": 0.9927073837739289, "support": 2194}, "\u23ce": {"f1-score": 0.2962962962962963, "precision": 0.7272727272727273, "recall": 0.18604651162790697, "support": 86}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9028571428571427, "precision": 0.9050736497545008, "recall": 0.9006514657980456, "support": 614}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 354}, "macro avg": {"f1-score": 0.18262515879220825, "precision": 0.25780693470663074, "recall": 0.16635828470432576, "support": 4146}, "micro avg": {"f1-score": 0.7757695566224231, "precision": 0.9356267029972752, "recall": 0.6625663289917993, "support": 4146}, "weighted avg": {"f1-score": 0.6890973485880725, "precision": 0.7555119513230528, "recall": 0.6625663289917993, "support": 4146}, "\u2205": {"f1-score": 0.9412273120138289, "precision": 0.9457229700390795, "recall": 0.9367741935483871, "support": 2325}, "\u23ce": {"f1-score": 0.12121212121212122, "precision": 0.7272727272727273, "recall": 0.06611570247933884, "support": 242}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u2423": {"f1-score": 0.7638121546961326, "precision": 0.9050736497545008, "recall": 0.6606929510155317, "support": 837}},
  "ppcr": 0.7081524360829715
}
```
</details>