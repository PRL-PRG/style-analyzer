# Train report for javascript / file:///tmp/top-repos-quality-repos-xn_ai3kh/myprecious-webtoon.git HEAD 1d37a9c9806131561b81ef312c453480956c0754

### Classification report

PPCR: 0.653

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.992| 0.744| 0.982| 0.843| 5917| 7896| 0.749 |
| `'` | 0.991| 1.000| 0.776| 0.995| 0.870| 2328| 3000| 0.776 |
| `␣` | 0.947| 0.954| 0.468| 0.950| 0.627| 2221| 4524| 0.491 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.968| 0.919| 0.831| 0.943| 0.894| 433| 479| 0.904 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.984| 0.885| 0.668| 0.932| 0.796| 355| 470| 0.755 |
| `⏎` | 0.948| 0.708| 0.198| 0.811| 0.328| 312| 1114| 0.280 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 158| 0.139 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 123| 0.033 |
| `macro avg` | 0.726| 0.682| 0.461| 0.702| 0.545| 11592| 17764| 0.653 |
| `weighted avg` | 0.968| 0.971| 0.633| 0.969| 0.747| 11592| 17764| 0.653 |
| `micro avg` | 0.971| 0.971| 0.633| 0.971| 0.767| 11592| 17764| 0.653 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1979 |5872 |40 |0 |0 |5 |0 |0 |0 |
|2303 |93 |2118 |0 |0 |5 |5 |0 |0 |
|672 |0 |0 |2328 |0 |0 |0 |0 |0 |
|802 |20 |68 |0 |221 |3 |0 |0 |0 |
|46 |23 |10 |0 |2 |398 |0 |0 |0 |
|115 |34 |0 |0 |7 |0 |314 |0 |0 |
|136 |0 |0 |22 |0 |0 |0 |0 |0 |
|119 |1 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| DjangoProject/staticfiles/admin/js/urlify.js | 80 |
| DjangoProject/staticfiles/admin/js/actions.js | 49 |
| DjangoProject/staticfiles/admin/js/core.js | 47 |
| DjangoProject/staticfiles/admin/js/inlines.js | 45 |
| DjangoProject/staticfiles/admin/js/SelectFilter2.js | 32 |
| DjangoProject/staticfiles/admin/js/admin/DateTimeShortcuts.js | 27 |
| DjangoProject/staticfiles/admin/js/SelectBox.js | 21 |
| DjangoProject/staticfiles/admin/js/calendar.js | 10 |
| DjangoProject/staticfiles/admin/js/admin/RelatedObjectLookups.js | 5 |
| DjangoProject/staticfiles/admin/js/collapse.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u0027": {"f1-score": 0.9952971355280035, "precision": 0.9906382978723405, "recall": 1.0, "support": 2328}, "macro avg": {"f1-score": 0.7016924863128039, "precision": 0.7263452485599375, "recall": 0.6822535318677173, "support": 11592}, "micro avg": {"f1-score": 0.970583160800552, "precision": 0.9705831608005521, "recall": 0.9705831608005521, "support": 11592}, "weighted avg": {"f1-score": 0.9687924598336922, "precision": 0.9682741718946878, "recall": 0.9705831608005521, "support": 11592}, "\u2205": {"f1-score": 0.9819397993311036, "precision": 0.9717027966241932, "recall": 0.9923947946594558, "support": 5917}, "\u23ce": {"f1-score": 0.8110091743119265, "precision": 0.9484978540772532, "recall": 0.7083333333333334, "support": 312}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9431279620853081, "precision": 0.9683698296836983, "recall": 0.9191685912240185, "support": 433}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9317507418397626, "precision": 0.9843260188087775, "recall": 0.8845070422535212, "support": 355}, "\u2423": {"f1-score": 0.9504150774063272, "precision": 0.947227191413238, "recall": 0.9536244934714093, "support": 2221}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "\u0027": {"f1-score": 0.8702803738317757, "precision": 0.9906382978723405, "recall": 0.776, "support": 3000}, "macro avg": {"f1-score": 0.5447373258376482, "precision": 0.7263452485599375, "recall": 0.4606505565150426, "support": 17764}, "micro avg": {"f1-score": 0.7665213244311215, "precision": 0.9705831608005521, "recall": 0.6333596036928619, "support": 17764}, "weighted avg": {"f1-score": 0.7468110757331881, "precision": 0.9520853961053714, "recall": 0.6333596036928619, "support": 17764}, "\u2205": {"f1-score": 0.8425281584044766, "precision": 0.9717027966241932, "recall": 0.7436676798378926, "support": 7896}, "\u23ce": {"f1-score": 0.3281365998515219, "precision": 0.9484978540772532, "recall": 0.19838420107719928, "support": 1114}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8943820224719102, "precision": 0.9683698296836983, "recall": 0.8308977035490606, "support": 479}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7959442332065907, "precision": 0.9843260188087775, "recall": 0.6680851063829787, "support": 470}, "\u2423": {"f1-score": 0.6266272189349112, "precision": 0.947227191413238, "recall": 0.46816976127320953, "support": 4524}},
  "ppcr": 0.6525557306912857
}
```
</details>
