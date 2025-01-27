# Test report for javascript / file:///tmp/top-repos-quality-repos-odcbryof/2019fall_42class_team1.git HEAD 0433a527ae869f44b4f342d0075f7a5ab82471b5

### Classification report

PPCR: 0.986

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.932| 0.979| 0.977| 0.955| 0.954| 4228| 4235| 0.998 |
| `␣` | 0.918| 0.904| 0.901| 0.911| 0.909| 1644| 1649| 0.997 |
| `'` | 0.943| 0.885| 0.879| 0.913| 0.910| 655| 660| 0.992 |
| `⏎` | 0.910| 0.837| 0.726| 0.872| 0.808| 349| 402| 0.868 |
| `⏎␣⁺␣⁺` | 0.949| 0.684| 0.684| 0.795| 0.795| 247| 247| 1.000 |
| `⏎␣⁻␣⁻` | 0.952| 0.785| 0.785| 0.861| 0.861| 228| 228| 1.000 |
| `"` | 0.769| 0.829| 0.825| 0.798| 0.796| 205| 206| 0.995 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 62| 0.403 |
| `weighted avg` | 0.923| 0.925| 0.912| 0.923| 0.913| 7581| 7689| 0.986 |
| `micro avg` | 0.925| 0.925| 0.912| 0.925| 0.919| 7581| 7689| 0.986 |
| `macro avg` | 0.797| 0.738| 0.722| 0.763| 0.754| 7581| 7689| 0.986 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7 |4139 |72 |0 |0 |0 |8 |9 |0 |
|5 |148 |1486 |0 |10 |0 |0 |0 |0 |
|5 |0 |23 |580 |1 |51 |0 |0 |0 |
|53 |36 |20 |0 |292 |0 |1 |0 |0 |
|1 |0 |0 |35 |0 |170 |0 |0 |0 |
|0 |66 |10 |0 |2 |0 |169 |0 |0 |
|0 |45 |4 |0 |0 |0 |0 |179 |0 |
|37 |5 |4 |0 |16 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.7981220657276995, "precision": 0.7692307692307693, "recall": 0.8292682926829268, "support": 205}, "\u0027": {"f1-score": 0.9133858267716536, "precision": 0.943089430894309, "recall": 0.8854961832061069, "support": 655}, "macro avg": {"f1-score": 0.7631195125685502, "precision": 0.7967263898647537, "recall": 0.7379477176744965, "support": 7581}, "micro avg": {"f1-score": 0.9253396649518533, "precision": 0.9253396649518533, "recall": 0.9253396649518533, "support": 7581}, "weighted avg": {"f1-score": 0.922616446870363, "precision": 0.9227923577149835, "recall": 0.9253396649518533, "support": 7581}, "\u2205": {"f1-score": 0.9551171108803507, "precision": 0.9324172110835773, "recall": 0.978949858088931, "support": 4228}, "\u23ce": {"f1-score": 0.8716417910447761, "precision": 0.9096573208722741, "recall": 0.836676217765043, "support": 349}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7952941176470588, "precision": 0.949438202247191, "recall": 0.6842105263157895, "support": 247}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8605769230769231, "precision": 0.9521276595744681, "recall": 0.7850877192982456, "support": 228}, "\u2423": {"f1-score": 0.9108182653999387, "precision": 0.9178505250154416, "recall": 0.9038929440389294, "support": 1644}},
  "cl_report_full": {"\"": {"f1-score": 0.7962529274004684, "precision": 0.7692307692307693, "recall": 0.8252427184466019, "support": 206}, "\u0027": {"f1-score": 0.9098039215686275, "precision": 0.943089430894309, "recall": 0.8787878787878788, "support": 660}, "macro avg": {"f1-score": 0.7541805551848568, "precision": 0.7967263898647537, "recall": 0.7222726218331428, "support": 7689}, "micro avg": {"f1-score": 0.9187950229207597, "precision": 0.9253396649518533, "recall": 0.9123423071920926, "support": 7689}, "weighted avg": {"f1-score": 0.9134035135565477, "precision": 0.9182600536961745, "recall": 0.9123423071920926, "support": 7689}, "\u2205": {"f1-score": 0.9543463223426331, "precision": 0.9324172110835773, "recall": 0.977331759149941, "support": 4235}, "\u23ce": {"f1-score": 0.8077455048409405, "precision": 0.9096573208722741, "recall": 0.7263681592039801, "support": 402}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7952941176470588, "precision": 0.949438202247191, "recall": 0.6842105263157895, "support": 247}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8605769230769231, "precision": 0.9521276595744681, "recall": 0.7850877192982456, "support": 228}, "\u2423": {"f1-score": 0.9094247246022031, "precision": 0.9178505250154416, "recall": 0.9011522134627047, "support": 1649}},
  "ppcr": 0.9859539602028873
}
```
</details>
