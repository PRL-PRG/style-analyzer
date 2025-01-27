# Test report for javascript / file:///tmp/top-repos-quality-repos-nh1i2qer/zero.git HEAD e6a72ee49e03008a64c86ee5c34e2120a3b52908

### Classification report

PPCR: 0.932

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.925| 0.962| 0.951| 0.943| 0.938| 2968| 3002| 0.989 |
| `␣` | 0.888| 0.906| 0.881| 0.897| 0.884| 1342| 1381| 0.972 |
| `"` | 0.997| 0.940| 0.911| 0.967| 0.952| 613| 632| 0.970 |
| `⏎␣⁺␣⁺` | 0.899| 0.789| 0.785| 0.841| 0.839| 204| 205| 0.995 |
| `⏎␣⁻␣⁻` | 0.798| 0.837| 0.811| 0.817| 0.804| 184| 190| 0.968 |
| `⏎` | 0.966| 0.389| 0.154| 0.554| 0.266| 144| 363| 0.397 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 91| 0.099 |
| `micro avg` | 0.918| 0.918| 0.856| 0.918| 0.886| 5464| 5864| 0.932 |
| `weighted avg` | 0.918| 0.918| 0.856| 0.915| 0.863| 5464| 5864| 0.932 |
| `macro avg` | 0.782| 0.689| 0.642| 0.717| 0.669| 5464| 5864| 0.932 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|34 |2855 |82 |0 |0 |2 |29 |0 |
|39 |100 |1216 |0 |0 |16 |10 |0 |
|19 |25 |12 |576 |0 |0 |0 |0 |
|219 |43 |45 |0 |56 |0 |0 |0 |
|1 |34 |7 |2 |0 |161 |0 |0 |
|6 |30 |0 |0 |0 |0 |154 |0 |
|82 |0 |7 |0 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9672544080604535, "precision": 0.9965397923875432, "recall": 0.9396411092985318, "support": 613}, "macro avg": {"f1-score": 0.7170750418844046, "precision": 0.7817873650798469, "recall": 0.6889628161543145, "support": 5464}, "micro avg": {"f1-score": 0.9183748169838947, "precision": 0.9183748169838946, "recall": 0.9183748169838946, "support": 5464}, "weighted avg": {"f1-score": 0.9146011704328647, "precision": 0.9182245431119385, "recall": 0.9183748169838946, "support": 5464}, "\u2205": {"f1-score": 0.9430222956234517, "precision": 0.9248461289277616, "recall": 0.9619272237196765, "support": 2968}, "\u23ce": {"f1-score": 0.5544554455445544, "precision": 0.9655172413793104, "recall": 0.3888888888888889, "support": 144}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8407310704960835, "precision": 0.8994413407821229, "recall": 0.7892156862745098, "support": 204}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8169761273209548, "precision": 0.7979274611398963, "recall": 0.8369565217391305, "support": 184}, "\u2423": {"f1-score": 0.8970859461453338, "precision": 0.8882395909422937, "recall": 0.9061102831594635, "support": 1342}},
  "cl_report_full": {"\"": {"f1-score": 0.9520661157024793, "precision": 0.9965397923875432, "recall": 0.9113924050632911, "support": 632}, "macro avg": {"f1-score": 0.6689912612660417, "precision": 0.7817873650798469, "recall": 0.6418726504569797, "support": 5864}, "micro avg": {"f1-score": 0.8859463276836157, "precision": 0.9183748169838946, "recall": 0.8557298772169167, "support": 5864}, "weighted avg": {"f1-score": 0.8627936830853088, "precision": 0.9071170794711767, "recall": 0.8557298772169167, "support": 5864}, "\u2205": {"f1-score": 0.9377566102808345, "precision": 0.9248461289277616, "recall": 0.9510326449033978, "support": 3002}, "\u23ce": {"f1-score": 0.2660332541567696, "precision": 0.9655172413793104, "recall": 0.15426997245179064, "support": 363}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8385416666666666, "precision": 0.8994413407821229, "recall": 0.7853658536585366, "support": 205}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8041775456919059, "precision": 0.7979274611398963, "recall": 0.8105263157894737, "support": 190}, "\u2423": {"f1-score": 0.8843636363636364, "precision": 0.8882395909422937, "recall": 0.8805213613323678, "support": 1381}},
  "ppcr": 0.931787175989086
}
```
</details>
