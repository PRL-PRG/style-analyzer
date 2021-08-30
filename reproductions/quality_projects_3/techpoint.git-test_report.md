# Test report for javascript / file:///tmp/top-repos-quality-repos-f55ya_6v/techpoint.git HEAD 911184c8b87b58843db9b96507f59d6a236da953

### Classification report

PPCR: 0.749

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.915| 0.961| 0.832| 0.938| 0.872| 1046| 1208| 0.866 |
| `␣` | 0.915| 0.860| 0.608| 0.886| 0.731| 585| 827| 0.707 |
| `'` | 0.992| 0.980| 0.980| 0.986| 0.986| 252| 252| 1.000 |
| `⏎␣⁺␣⁺` | 0.743| 0.788| 0.371| 0.765| 0.495| 33| 70| 0.471 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 100| 0.080 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 65| 0.108 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 58| 0.017 |
| `weighted avg` | 0.915| 0.922| 0.690| 0.918| 0.752| 1932| 2580| 0.749 |
| `micro avg` | 0.922| 0.922| 0.690| 0.922| 0.789| 1932| 2580| 0.749 |
| `macro avg` | 0.509| 0.513| 0.399| 0.511| 0.440| 1932| 2580| 0.749 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|162 |1005 |41 |0 |0 |0 |0 |0 |
|242 |71 |503 |2 |0 |9 |0 |0 |
|0 |5 |0 |247 |0 |0 |0 |0 |
|92 |4 |4 |0 |0 |0 |0 |0 |
|37 |6 |1 |0 |0 |26 |0 |0 |
|58 |7 |0 |0 |0 |0 |0 |0 |
|57 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9860279441117764, "precision": 0.9919678714859438, "recall": 0.9801587301587301, "support": 252}, "macro avg": {"f1-score": 0.5106539198285028, "precision": 0.5092387164766612, "recall": 0.5126670910200001, "support": 1932}, "micro avg": {"f1-score": 0.9218426501035196, "precision": 0.9218426501035196, "recall": 0.9218426501035196, "support": 1932}, "weighted avg": {"f1-score": 0.9176244043735373, "precision": 0.914546403634852, "recall": 0.9218426501035196, "support": 1932}, "\u2205": {"f1-score": 0.9375000000000001, "precision": 0.9153005464480874, "recall": 0.9608030592734226, "support": 1046}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7647058823529412, "precision": 0.7428571428571429, "recall": 0.7878787878787878, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.8863436123348019, "precision": 0.9145454545454546, "recall": 0.8598290598290599, "support": 585}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9860279441117764, "precision": 0.9919678714859438, "recall": 0.9801587301587301, "support": 252}, "macro avg": {"f1-score": 0.4404969931996094, "precision": 0.5092387164766612, "recall": 0.398823347843212, "support": 2580}, "micro avg": {"f1-score": 0.7894503546099291, "precision": 0.9218426501035196, "recall": 0.6903100775193799, "support": 2580}, "weighted avg": {"f1-score": 0.7520427611108881, "precision": 0.8387550599352087, "recall": 0.6903100775193799, "support": 2580}, "\u2205": {"f1-score": 0.8716392020815266, "precision": 0.9153005464480874, "recall": 0.831953642384106, "support": 1208}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.49523809523809526, "precision": 0.7428571428571429, "recall": 0.37142857142857144, "support": 70}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u2423": {"f1-score": 0.7305737109658678, "precision": 0.9145454545454546, "recall": 0.6082224909310762, "support": 827}},
  "ppcr": 0.7488372093023256
}
```
</details>