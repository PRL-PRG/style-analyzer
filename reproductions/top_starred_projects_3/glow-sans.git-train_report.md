# Train report for javascript / file:///tmp/top-repos-quality-repos-wqaaepdw/glow-sans.git HEAD 52434233ac43f40be3b28730689339d19c8f2964

### Classification report

PPCR: 0.767

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 0.995| 0.951| 0.978| 0.956| 12905| 13509| 0.955 |
| `␣` | 0.963| 0.930| 0.619| 0.947| 0.754| 4593| 6898| 0.666 |
| `'` | 1.000| 1.000| 0.514| 1.000| 0.679| 600| 1168| 0.514 |
| `"` | 1.000| 1.000| 0.497| 1.000| 0.664| 425| 855| 0.497 |
| `⏎` | 0.930| 0.535| 0.115| 0.680| 0.204| 325| 1519| 0.214 |
| `⏎␣⁻␣⁻` | 0.912| 0.901| 0.520| 0.906| 0.663| 242| 419| 0.578 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 132| 494| 0.267 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 226| 0.097 |
| `weighted avg` | 0.955| 0.963| 0.739| 0.958| 0.800| 19244| 25088| 0.767 |
| `micro avg` | 0.963| 0.963| 0.739| 0.963| 0.836| 19244| 25088| 0.767 |
| `macro avg` | 0.721| 0.670| 0.402| 0.689| 0.490| 19244| 25088| 0.767 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|604 |12843 |61 |0 |0 |0 |0 |1 |0 |
|2305 |289 |4273 |11 |0 |0 |0 |20 |0 |
|1194 |79 |72 |174 |0 |0 |0 |0 |0 |
|568 |0 |0 |0 |600 |0 |0 |0 |0 |
|430 |0 |0 |0 |0 |425 |0 |0 |0 |
|362 |104 |28 |0 |0 |0 |0 |0 |0 |
|177 |23 |1 |0 |0 |0 |0 |218 |0 |
|204 |20 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/glyph-manipulate/PostFilters.js | 78 |
| demo/GlyphPreviewPanel.js | 72 |
| src/font-build/build-font.js | 54 |
| src/font-build/res/features.js | 50 |
| src/font-build/tables/name.js | 45 |
| demo/index.js | 42 |
| demo/resources.js | 35 |
| src/font-build/glyph-filters.js | 30 |
| src/utils/point-math.js | 29 |
| scripts/extract-han-models.js | 28 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 425}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 600}, "macro avg": {"f1-score": 0.6888466099314683, "precision": 0.7209417347176068, "recall": 0.67021693542757, "support": 19244}, "micro avg": {"f1-score": 0.9630534192475577, "precision": 0.9630534192475577, "recall": 0.9630534192475577, "support": 19244}, "weighted avg": {"f1-score": 0.9579358012477821, "precision": 0.9551463453403244, "recall": 0.9630534192475577, "support": 19244}, "\u2205": {"f1-score": 0.9780299280356395, "precision": 0.961446324300045, "recall": 0.995195660596668, "support": 12905}, "\u23ce": {"f1-score": 0.6796875, "precision": 0.93048128342246, "recall": 0.5353846153846153, "support": 325}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9064449064449066, "precision": 0.9121338912133892, "recall": 0.9008264462809917, "support": 242}, "\u2423": {"f1-score": 0.9466105449712007, "precision": 0.9634723788049605, "recall": 0.9303287611582843, "support": 4593}},
  "cl_report_full": {"\"": {"f1-score": 0.6640625, "precision": 1.0, "recall": 0.49707602339181284, "support": 855}, "\u0027": {"f1-score": 0.6787330316742082, "precision": 1.0, "recall": 0.5136986301369864, "support": 1168}, "macro avg": {"f1-score": 0.48993989710282976, "precision": 0.7209417347176068, "recall": 0.4019705679058755, "support": 25088}, "micro avg": {"f1-score": 0.8361003338446269, "precision": 0.9630534192475577, "recall": 0.7387197066326531, "support": 25088}, "weighted avg": {"f1-score": 0.7997789974506774, "precision": 0.9348212704840183, "recall": 0.7387197066326531, "support": 25088}, "\u2205": {"f1-score": 0.9560427289983996, "precision": 0.961446324300045, "recall": 0.9506995336442372, "support": 13509}, "\u23ce": {"f1-score": 0.20398593200468934, "precision": 0.93048128342246, "recall": 0.11454904542462147, "support": 1519}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 226}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 494}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.662613981762918, "precision": 0.9121338912133892, "recall": 0.5202863961813843, "support": 419}, "\u2423": {"f1-score": 0.754081002382423, "precision": 0.9634723788049605, "recall": 0.6194549144679617, "support": 6898}},
  "ppcr": 0.7670599489795918
}
```
</details>
