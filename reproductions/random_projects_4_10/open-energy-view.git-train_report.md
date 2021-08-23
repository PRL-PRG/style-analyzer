# Train report for javascript / file:///tmp/top-repos-quality-repos-x13ev0qx/open-energy-view.git HEAD e4265dc4c240ec35d8fa4d3d3fc94c98e18276bf

### Classification report

PPCR: 0.544

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.951| 0.993| 0.831| 0.971| 0.887| 4361| 5212| 0.837 |
| `␣` | 0.954| 0.892| 0.205| 0.922| 0.338| 742| 3222| 0.230 |
| `"` | 1.000| 1.000| 0.806| 1.000| 0.893| 541| 671| 0.806 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 72| 307| 0.235 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 714| 0.056 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 334| 0.099 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 180| 0.000 |
| `macro avg` | 0.415| 0.412| 0.263| 0.413| 0.302| 5789| 10640| 0.544 |
| `weighted avg` | 0.932| 0.956| 0.520| 0.943| 0.593| 5789| 10640| 0.544 |
| `micro avg` | 0.956| 0.956| 0.520| 0.956| 0.673| 5789| 10640| 0.544 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|851 |4329 |32 |0 |0 |0 |0 |0 |
|2480 |80 |662 |0 |0 |0 |0 |0 |
|674 |40 |0 |0 |0 |0 |0 |0 |
|130 |0 |0 |0 |541 |0 |0 |0 |
|301 |33 |0 |0 |0 |0 |0 |0 |
|235 |72 |0 |0 |0 |0 |0 |0 |
|180 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| open_energy_view/frontend/js/data-structures/EnergyHistory.js | 43 |
| open_energy_view/frontend/js/api/DatabaseService.js | 22 |
| open_energy_view/frontend/js/tests/performance.js | 22 |
| open_energy_view/frontend/js/functions/indexInDb.js | 13 |
| open_energy_view/frontend/js/api/helpers.js | 12 |
| open_energy_view/frontend/js/functions/groupBy.js | 11 |
| open_energy_view/frontend/js/data-structures/helpers/sumPartitions.js | 11 |
| open_energy_view/frontend/js/data-structures/helpers/makeColorsArray.js | 10 |
| open_energy_view/frontend/js/functions/makeIntervalArray.js | 9 |
| open_energy_view/frontend/js/components/Trends/trend.worker.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 541}, "macro avg": {"f1-score": 0.4133111075352642, "precision": 0.4149261964698627, "recall": 0.41212078883462894, "support": 5789}, "micro avg": {"f1-score": 0.9556054586284333, "precision": 0.9556054586284333, "recall": 0.9556054586284333, "support": 5789}, "weighted avg": {"f1-score": 0.9432389045032168, "precision": 0.9318228220138849, "recall": 0.9556054586284333, "support": 5789}, "\u2205": {"f1-score": 0.9711721817162087, "precision": 0.950592885375494, "recall": 0.9926622334326989, "support": 4361}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u2423": {"f1-score": 0.9220055710306407, "precision": 0.9538904899135446, "recall": 0.8921832884097035, "support": 742}},
  "cl_report_full": {"\"": {"f1-score": 0.8927392739273927, "precision": 1.0, "recall": 0.8062593144560357, "support": 671}, "macro avg": {"f1-score": 0.30248350467691754, "precision": 0.4149261964698627, "recall": 0.2631864327886147, "support": 10640}, "micro avg": {"f1-score": 0.6734433014790919, "precision": 0.9556054586284333, "recall": 0.5199248120300752, "support": 10640}, "weighted avg": {"f1-score": 0.5929567611334331, "precision": 0.8175681651389584, "recall": 0.5199248120300752, "support": 10640}, "\u2205": {"f1-score": 0.886545156665984, "precision": 0.950592885375494, "recall": 0.8305832693783577, "support": 5212}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 714}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 334}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 307}, "\u2423": {"f1-score": 0.338100102145046, "precision": 0.9538904899135446, "recall": 0.20546244568590938, "support": 3222}},
  "ppcr": 0.5440789473684211
}
```
</details>
