# Train report for javascript / file:///tmp/top-repos-quality-repos-chtdai7c/tiny-tools.git HEAD 2af2c0ed405ded5013d1902ca048a64bd9bb4f39

### Classification report

PPCR: 0.515

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.936| 0.999| 0.851| 0.966| 0.891| 26664| 31314| 0.852 |
| `␣` | 0.979| 0.686| 0.116| 0.807| 0.208| 3693| 21774| 0.170 |
| `⏎␣⁺␣⁺` | 0.925| 0.913| 0.830| 0.919| 0.875| 2610| 2871| 0.909 |
| `'` | 1.000| 1.000| 0.347| 1.000| 0.515| 783| 2259| 0.347 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 487| 2744| 0.177 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 177| 4542| 0.039 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 1284| 0.010 |
| `macro avg` | 0.548| 0.514| 0.306| 0.527| 0.356| 34427| 66788| 0.515 |
| `weighted avg` | 0.923| 0.940| 0.484| 0.927| 0.541| 34427| 66788| 0.515 |
| `micro avg` | 0.940| 0.940| 0.484| 0.940| 0.639| 34427| 66788| 0.515 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|4650 |26647 |17 |0 |0 |0 |0 |0 |
|18081 |978 |2533 |0 |0 |182 |0 |0 |
|1476 |0 |0 |783 |0 |0 |0 |0 |
|4365 |140 |30 |0 |0 |7 |0 |0 |
|261 |227 |0 |0 |0 |2383 |0 |0 |
|2257 |482 |0 |0 |0 |5 |0 |0 |
|1271 |6 |7 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| health_prove_collector/static/vue.js | 2000 |
| health_prove_collector/static/axios.js | 81 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 783}, "macro avg": {"f1-score": 0.5274249887160016, "precision": 0.5484977301850354, "recall": 0.5140402121010683, "support": 34427}, "micro avg": {"f1-score": 0.9395532576175676, "precision": 0.9395532576175676, "recall": 0.9395532576175676, "support": 34427}, "weighted avg": {"f1-score": 0.9274616219010536, "precision": 0.9225407096203233, "recall": 0.9395532576175676, "support": 34427}, "\u2205": {"f1-score": 0.9664514725083417, "precision": 0.9356390449438202, "recall": 0.9993624362436243, "support": 26664}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 177}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9188355504144978, "precision": 0.9247186651144742, "recall": 0.9130268199233716, "support": 2610}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 487}, "\u2423": {"f1-score": 0.806687898089172, "precision": 0.979126401236954, "recall": 0.685892228540482, "support": 3693}},
  "cl_report_full": {"\u0027": {"f1-score": 0.514792899408284, "precision": 1.0, "recall": 0.3466135458167331, "support": 2259}, "macro avg": {"f1-score": 0.3555511607249851, "precision": 0.5484977301850354, "recall": 0.3062757947700926, "support": 66788}, "micro avg": {"f1-score": 0.6391542755520427, "precision": 0.9395532576175676, "recall": 0.48430855842366893, "support": 66788}, "weighted avg": {"f1-score": 0.5407033826801956, "precision": 0.8314662304822406, "recall": 0.48430855842366893, "support": 66788}, "\u2205": {"f1-score": 0.8912934408134595, "precision": 0.9356390449438202, "recall": 0.8509612313980967, "support": 31314}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4542}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1284}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8748164464023496, "precision": 0.9247186651144742, "recall": 0.8300243817485197, "support": 2871}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2744}, "\u2423": {"f1-score": 0.2079553384508025, "precision": 0.979126401236954, "recall": 0.11633140442729861, "support": 21774}},
  "ppcr": 0.515466850332395
}
```
</details>
