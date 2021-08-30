# Test report for javascript / file:///tmp/top-repos-quality-repos-a8rzqgwl/qgis-webappbuilder-plugin.git HEAD ceed00caa091ca875feef624c1cf4a12bb23cda4

### Classification report

PPCR: 0.982

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.829| 0.965| 0.959| 0.892| 0.889| 1284| 1292| 0.994 |
| `␣` | 0.805| 0.633| 0.624| 0.709| 0.703| 436| 442| 0.986 |
| `"` | 0.938| 0.917| 0.917| 0.927| 0.927| 132| 132| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 76| 96| 0.792 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 1.000| 0.463| 0.463| 0.633| 0.633| 41| 41| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.966| 0.778| 0.778| 0.862| 0.862| 36| 36| 1.000 |
| `⏎⏎` | 0.800| 0.480| 0.429| 0.600| 0.558| 25| 28| 0.893 |
| `weighted avg` | 0.805| 0.835| 0.820| 0.812| 0.800| 2030| 2067| 0.982 |
| `micro avg` | 0.835| 0.835| 0.820| 0.835| 0.827| 2030| 2067| 0.982 |
| `macro avg` | 0.762| 0.605| 0.596| 0.660| 0.653| 2030| 2067| 0.982 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|8 |1239 |45 |0 |0 |0 |0 |0 |
|6 |153 |276 |7 |0 |0 |0 |0 |
|0 |9 |2 |121 |0 |0 |0 |0 |
|0 |20 |2 |0 |19 |0 |0 |0 |
|0 |7 |1 |0 |0 |28 |0 |0 |
|3 |9 |4 |0 |0 |0 |12 |0 |
|20 |58 |13 |1 |0 |1 |3 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9272030651340997, "precision": 0.937984496124031, "recall": 0.9166666666666666, "support": 132}, "macro avg": {"f1-score": 0.6603376125221219, "precision": 0.7624184289059187, "recall": 0.6051199817935148, "support": 2030}, "micro avg": {"f1-score": 0.8349753694581281, "precision": 0.8349753694581281, "recall": 0.8349753694581281, "support": 2030}, "weighted avg": {"f1-score": 0.8119456866186986, "precision": 0.8051908852704135, "recall": 0.8349753694581281, "support": 2030}, "\u2205": {"f1-score": 0.8916876574307304, "precision": 0.82876254180602, "recall": 0.9649532710280374, "support": 1284}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u23ce\u23ce": {"f1-score": 0.6, "precision": 0.8, "recall": 0.48, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.6333333333333334, "precision": 1.0, "recall": 0.4634146341463415, "support": 41}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8615384615384615, "precision": 0.9655172413793104, "recall": 0.7777777777777778, "support": 36}, "\u2423": {"f1-score": 0.7086007702182285, "precision": 0.8046647230320699, "recall": 0.6330275229357798, "support": 436}},
  "cl_report_full": {"\"": {"f1-score": 0.9272030651340997, "precision": 0.937984496124031, "recall": 0.9166666666666666, "support": 132}, "macro avg": {"f1-score": 0.6532181718558461, "precision": 0.7624184289059187, "recall": 0.5956918892108372, "support": 2067}, "micro avg": {"f1-score": 0.8274347083231632, "precision": 0.8349753694581281, "recall": 0.8200290275761973, "support": 2067}, "weighted avg": {"f1-score": 0.800465362628911, "precision": 0.7974821411570296, "recall": 0.8200290275761973, "support": 2067}, "\u2205": {"f1-score": 0.8891280947255112, "precision": 0.82876254180602, "recall": 0.9589783281733746, "support": 1292}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u23ce": {"f1-score": 0.5581395348837209, "precision": 0.8, "recall": 0.42857142857142855, "support": 28}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.6333333333333334, "precision": 1.0, "recall": 0.4634146341463415, "support": 41}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8615384615384615, "precision": 0.9655172413793104, "recall": 0.7777777777777778, "support": 36}, "\u2423": {"f1-score": 0.703184713375796, "precision": 0.8046647230320699, "recall": 0.6244343891402715, "support": 442}},
  "ppcr": 0.9820996613449444
}
```
</details>