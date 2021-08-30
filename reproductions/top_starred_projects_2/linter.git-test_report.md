# Test report for javascript / file:///tmp/top-repos-quality-repos-sbvnzbri/linter.git HEAD ba3fbaf9d7b741e3432a05bca2d6178169f36cd4

### Classification report

PPCR: 0.990

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.976| 0.966| 0.975| 0.970| 2624| 2650| 0.990 |
| `␣` | 0.774| 0.633| 0.626| 0.696| 0.692| 618| 625| 0.989 |
| `⏎` | 0.657| 0.832| 0.832| 0.734| 0.734| 322| 322| 1.000 |
| `'` | 0.994| 0.970| 0.932| 0.982| 0.962| 169| 176| 0.960 |
| `⏎␣⁺␣⁺` | 0.586| 0.967| 0.967| 0.730| 0.730| 60| 60| 1.000 |
| `⏎␣⁻␣⁻` | 0.625| 1.000| 1.000| 0.769| 0.769| 60| 60| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 47| 1.000 |
| `micro avg` | 0.898| 0.898| 0.889| 0.898| 0.893| 3900| 3940| 0.990 |
| `macro avg` | 0.659| 0.768| 0.760| 0.698| 0.694| 3900| 3940| 0.990 |
| `weighted avg` | 0.894| 0.898| 0.889| 0.893| 0.888| 3900| 3940| 0.990 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|26 |2560 |57 |4 |1 |0 |2 |0 |
|7 |56 |391 |96 |0 |41 |34 |0 |
|0 |10 |44 |268 |0 |0 |0 |0 |
|7 |0 |5 |0 |164 |0 |0 |0 |
|0 |0 |2 |0 |0 |58 |0 |0 |
|0 |0 |0 |0 |0 |0 |60 |0 |
|0 |1 |6 |40 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9820359281437127, "precision": 0.9939393939393939, "recall": 0.9704142011834319, "support": 169}, "macro avg": {"f1-score": 0.6980677795894329, "precision": 0.6586305390030771, "recall": 0.7682392635337167, "support": 3900}, "micro avg": {"f1-score": 0.8976923076923077, "precision": 0.8976923076923077, "recall": 0.8976923076923077, "support": 3900}, "weighted avg": {"f1-score": 0.8926153958441935, "precision": 0.8942832509975905, "recall": 0.8976923076923077, "support": 3900}, "\u2205": {"f1-score": 0.9750523709769567, "precision": 0.9744956223829463, "recall": 0.975609756097561, "support": 2624}, "\u23ce": {"f1-score": 0.7342465753424658, "precision": 0.6568627450980392, "recall": 0.8322981366459627, "support": 322}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.729559748427673, "precision": 0.5858585858585859, "recall": 0.9666666666666667, "support": 60}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7692307692307693, "precision": 0.625, "recall": 1.0, "support": 60}, "\u2423": {"f1-score": 0.6963490650044525, "precision": 0.7742574257425743, "recall": 0.6326860841423948, "support": 618}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9618768328445747, "precision": 0.9939393939393939, "recall": 0.9318181818181818, "support": 176}, "macro avg": {"f1-score": 0.6938853673122388, "precision": 0.6586305390030771, "recall": 0.7603458172828382, "support": 3940}, "micro avg": {"f1-score": 0.8931122448979592, "precision": 0.8976923076923077, "recall": 0.8885786802030456, "support": 3940}, "weighted avg": {"f1-score": 0.8881535862020298, "precision": 0.8947763814239426, "recall": 0.8885786802030456, "support": 3940}, "\u2205": {"f1-score": 0.9702482471101004, "precision": 0.9744956223829463, "recall": 0.9660377358490566, "support": 2650}, "\u23ce": {"f1-score": 0.7342465753424658, "precision": 0.6568627450980392, "recall": 0.8322981366459627, "support": 322}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.729559748427673, "precision": 0.5858585858585859, "recall": 0.9666666666666667, "support": 60}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7692307692307693, "precision": 0.625, "recall": 1.0, "support": 60}, "\u2423": {"f1-score": 0.6920353982300885, "precision": 0.7742574257425743, "recall": 0.6256, "support": 625}},
  "ppcr": 0.9898477157360406
}
```
</details>