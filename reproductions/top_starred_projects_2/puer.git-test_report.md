# Test report for javascript / file:///tmp/top-repos-quality-repos-ksif969j/puer.git HEAD c5ea5fc9d6f823ccbc66dfcc901a084d22f1d2ef

### Classification report

PPCR: 0.947

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.873| 0.918| 0.909| 0.895| 0.891| 3568| 3603| 0.990 |
| `␣` | 0.763| 0.848| 0.840| 0.803| 0.800| 2098| 2118| 0.991 |
| `'` | 0.838| 0.662| 0.513| 0.740| 0.636| 429| 554| 0.774 |
| `⏎` | 0.784| 0.521| 0.397| 0.626| 0.527| 307| 403| 0.762 |
| `⏎␣⁺␣⁺` | 0.921| 0.897| 0.897| 0.909| 0.909| 194| 194| 1.000 |
| `⏎␣⁻␣⁻` | 0.969| 0.550| 0.454| 0.701| 0.618| 171| 207| 0.826 |
| `⏎⏎` | 0.600| 0.074| 0.054| 0.132| 0.099| 81| 111| 0.730 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 73| 116| 0.629 |
| `micro avg` | 0.834| 0.834| 0.790| 0.834| 0.811| 6921| 7306| 0.947 |
| `macro avg` | 0.718| 0.559| 0.508| 0.601| 0.560| 6921| 7306| 0.947 |
| `weighted avg` | 0.825| 0.834| 0.790| 0.823| 0.792| 6921| 7306| 0.947 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|35 |3275 |290 |0 |0 |0 |3 |0 |0 |
|20 |297 |1779 |3 |5 |0 |11 |3 |0 |
|96 |22 |121 |160 |0 |4 |0 |0 |0 |
|125 |61 |83 |0 |284 |0 |1 |0 |0 |
|30 |8 |31 |36 |0 |6 |0 |0 |0 |
|0 |4 |15 |1 |0 |0 |174 |0 |0 |
|36 |70 |3 |4 |0 |0 |0 |94 |0 |
|43 |13 |10 |0 |50 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 73}, "\u0027": {"f1-score": 0.7395833333333333, "precision": 0.8377581120943953, "recall": 0.662004662004662, "support": 429}, "macro avg": {"f1-score": 0.6007496058263567, "precision": 0.7184970938122336, "recall": 0.5587122235310635, "support": 6921}, "micro avg": {"f1-score": 0.8339835283918509, "precision": 0.8339835283918509, "recall": 0.8339835283918509, "support": 6921}, "weighted avg": {"f1-score": 0.8228608901733813, "precision": 0.8249732842597142, "recall": 0.8339835283918509, "support": 6921}, "\u2205": {"f1-score": 0.8950532932495218, "precision": 0.8733333333333333, "recall": 0.9178811659192825, "support": 3568}, "\u23ce": {"f1-score": 0.6262230919765166, "precision": 0.7843137254901961, "recall": 0.5211726384364821, "support": 307}, "\u23ce\u23ce": {"f1-score": 0.13186813186813187, "precision": 0.6, "recall": 0.07407407407407407, "support": 81}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9086161879895561, "precision": 0.9206349206349206, "recall": 0.8969072164948454, "support": 194}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7014925373134329, "precision": 0.9690721649484536, "recall": 0.5497076023391813, "support": 171}, "\u2423": {"f1-score": 0.8031602708803612, "precision": 0.7628644939965694, "recall": 0.847950428979981, "support": 2098}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u0027": {"f1-score": 0.6360582306830908, "precision": 0.8377581120943953, "recall": 0.5126353790613718, "support": 554}, "macro avg": {"f1-score": 0.5599744158793543, "precision": 0.7184970938122336, "recall": 0.507954169585226, "support": 7306}, "micro avg": {"f1-score": 0.8114149153018907, "precision": 0.8339835283918509, "recall": 0.7900355871886121, "support": 7306}, "weighted avg": {"f1-score": 0.7915547127733655, "precision": 0.8196502239946732, "recall": 0.7900355871886121, "support": 7306}, "\u2205": {"f1-score": 0.8907928736570108, "precision": 0.8733333333333333, "recall": 0.9089647515958923, "support": 3603}, "\u23ce": {"f1-score": 0.5271828665568369, "precision": 0.7843137254901961, "recall": 0.3970223325062035, "support": 403}, "\u23ce\u23ce": {"f1-score": 0.09917355371900827, "precision": 0.6, "recall": 0.05405405405405406, "support": 111}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9086161879895561, "precision": 0.9206349206349206, "recall": 0.8969072164948454, "support": 194}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6184210526315791, "precision": 0.9690721649484536, "recall": 0.45410628019323673, "support": 207}, "\u2423": {"f1-score": 0.7995505617977527, "precision": 0.7628644939965694, "recall": 0.839943342776204, "support": 2118}},
  "ppcr": 0.9473035860936216
}
```
</details>
