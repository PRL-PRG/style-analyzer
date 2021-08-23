# Test report for javascript / file:///tmp/top-repos-quality-repos-av4ezdeu/weather-10kb-wxkb.git HEAD f3f17b417d411d50c633c25aea7ccbf624d80203

### Classification report

PPCR: 0.628

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.929| 0.987| 0.765| 0.957| 0.839| 395| 510| 0.775 |
| `'` | 0.952| 0.851| 0.426| 0.899| 0.588| 47| 94| 0.500 |
| `␣` | 0.914| 1.000| 0.286| 0.955| 0.435| 32| 112| 0.286 |
| `⏎␣⁺␣⁺` | 1.000| 0.867| 0.765| 0.929| 0.867| 30| 34| 0.882 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 61| 0.164 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 22| 0.409 |
| `macro avg` | 0.633| 0.618| 0.373| 0.623| 0.455| 523| 833| 0.628 |
| `weighted avg` | 0.900| 0.933| 0.586| 0.915| 0.674| 523| 833| 0.628 |
| `micro avg` | 0.933| 0.933| 0.586| 0.933| 0.720| 523| 833| 0.628 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|115 |390 |3 |2 |0 |0 |0 |
|80 |0 |32 |0 |0 |0 |0 |
|47 |7 |0 |40 |0 |0 |0 |
|51 |10 |0 |0 |0 |0 |0 |
|4 |4 |0 |0 |0 |26 |0 |
|13 |9 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.898876404494382, "precision": 0.9523809523809523, "recall": 0.851063829787234, "support": 47}, "macro avg": {"f1-score": 0.623287821397792, "precision": 0.6325396825396825, "recall": 0.6175120447676332, "support": 523}, "micro avg": {"f1-score": 0.9330783938814532, "precision": 0.9330783938814532, "recall": 0.9330783938814532, "support": 523}, "weighted avg": {"f1-score": 0.9153122521289373, "precision": 0.9002003095693344, "recall": 0.9330783938814532, "support": 523}, "\u2205": {"f1-score": 0.9570552147239264, "precision": 0.9285714285714286, "recall": 0.9873417721518988, "support": 395}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9285714285714286, "precision": 1.0, "recall": 0.8666666666666667, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.955223880597015, "precision": 0.9142857142857143, "recall": 1.0, "support": 32}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5882352941176471, "precision": 0.9523809523809523, "recall": 0.425531914893617, "support": 94}, "macro avg": {"f1-score": 0.4548309646439221, "precision": 0.6325396825396825, "recall": 0.37344299421896415, "support": 833}, "micro avg": {"f1-score": 0.7197640117994101, "precision": 0.9330783938814532, "recall": 0.5858343337334934, "support": 833}, "weighted avg": {"f1-score": 0.6737870642971203, "precision": 0.8397301777853999, "recall": 0.5858343337334934, "support": 833}, "\u2205": {"f1-score": 0.8387096774193549, "precision": 0.9285714285714286, "recall": 0.7647058823529411, "support": 510}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8666666666666666, "precision": 1.0, "recall": 0.7647058823529411, "support": 34}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u2423": {"f1-score": 0.43537414965986393, "precision": 0.9142857142857143, "recall": 0.2857142857142857, "support": 112}},
  "ppcr": 0.6278511404561825
}
```
</details>
